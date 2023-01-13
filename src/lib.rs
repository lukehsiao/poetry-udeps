use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{self, Read},
    path::{Path, PathBuf},
    thread,
};

use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use ignore::{types::TypesBuilder, WalkBuilder};
use log::info;
use toml::Value;
use xshell::{cmd, Shell};

mod name_map;
mod parser;
use crate::name_map::KNOWN_NAMES;
use crate::parser::parse_python_file;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: Verbosity,
}

fn get_venv_path() -> Result<String> {
    let sh = Shell::new()?;

    Ok(cmd!(sh, "poetry env info -p").quiet().read()?)
}

enum DepType {
    Main,
    Dev,
}

/// Returns two maps (one for core deps, one for dev-deps).
///
/// The maps are filled with either the original package name -> None, or with the alias ->
/// Some(package name). This helps us quickly determine which original dependency to eliminate if either
/// the original package name or alias is found.
///
/// We do not simply track the aliases alone, as reporting an alias as obsolete is not as
/// straightforward to the user which line to eliminate from their pyproject.toml.
fn get_dependencies(file: &Path, deps: DepType) -> Result<BTreeMap<String, Option<String>>> {
    let sh = Shell::new()?;
    let toml = fs::read_to_string(file)?;

    // TODO: map package name to actual module name.
    // Ref: https://stackoverflow.com/a/54853084
    let value = toml.parse::<Value>()?;
    let dep_key = match deps {
        DepType::Main => "dependencies",
        DepType::Dev => "dev-dependencies",
    };
    let mut dependencies: BTreeMap<String, Option<String>> = BTreeMap::new();

    // Generate a list of possible aliases for the package
    value
        .get("tool")
        .unwrap()
        .get("poetry")
        .unwrap()
        .get(dep_key)
        .unwrap()
        .as_table()
        .unwrap()
        .keys()
        .for_each(|s| {
            let package = String::from(s);
            dependencies.insert(package.clone(), None);
            let mut alias = KNOWN_NAMES.get(&package).map(|a| String::from(*a));

            // Try to grab from top_level.txt
            {
                let bash = format!(r#"cat $(poetry run python -c "import pkg_resources; print(pkg_resources.get_distribution('{}').egg_info)" 2>/dev/null )/top_level.txt 2> /dev/null"#, package);
                cmd!(sh, "bash -c {bash}").quiet().read().unwrap_or(String::new()).split_whitespace().for_each(|a| {
                    dependencies.insert(String::from(a), Some(package.clone()));
                })
            }

            // Or basic replacement
            if alias.is_none() && package.contains("-") {
                alias = Some(package.replace("-", "_").to_lowercase())
            }
            if let Some(a) = alias {
                dependencies.insert(a, Some(package));
            } else {
                dependencies.insert(package, None);
            }
        });
    Ok(dependencies)
}

pub fn run(_cli: Cli) -> Result<()> {
    let pyproject_path = Path::new("pyproject.toml");
    let mut main_deps = get_dependencies(pyproject_path, DepType::Main)?;
    info!("Main Deps: {:#?}", main_deps);
    let mut dev_deps = get_dependencies(pyproject_path, DepType::Dev)?;
    info!("Dev Deps: {:#?}", dev_deps);

    let venv_path = get_venv_path()?;
    info!("Reading files in venv: {}", venv_path);

    let (tx, rx) = crossbeam_channel::bounded::<(String, PathBuf)>(100);

    // Setup main thread for stdout
    let stdout_thread = thread::spawn(move || -> io::Result<()> {
        for (package, path) in rx {
            if main_deps.contains_key(&package) {
                if let Some(v) = main_deps.remove(&package) {
                    match v {
                        Some(orig) => {
                            info!("Found {} in {}", orig, path.display());
                            main_deps.remove(&orig);
                        }
                        None => info!("Found {} in {}", package, path.display()),
                    }
                }
            }
            if dev_deps.contains_key(&package) {
                if let Some(v) = dev_deps.remove(&package) {
                    match v {
                        Some(orig) => {
                            info!("Found {} in {}", orig, path.display());
                            dev_deps.remove(&orig);
                        }
                        None => info!("Found {} in {}", package, path.display()),
                    }
                }
            }
        }

        for (key, value) in main_deps.iter() {
            // Only print the non-alias names
            if value.is_none() {
                println!("{}", key)
            }
        }

        Ok(())
    });

    // Iterate over Python files in parallel in the venv
    let types = TypesBuilder::new().add_defaults().select("py").build()?;
    let walker = WalkBuilder::new(venv_path)
        .standard_filters(false)
        .types(types)
        .build_parallel();
    walker.run(|| {
        let tx = tx.clone();
        Box::new(move |result| {
            use ignore::WalkState::Continue;

            if let Ok(dir) = result {
                if dir.file_type().unwrap().is_file() {
                    let mut file = File::open(dir.path()).unwrap();
                    let mut buf = Vec::new();
                    file.read_to_end(&mut buf).unwrap();
                    let contents = String::from_utf8_lossy(&buf);
                    let v = parse_python_file(&contents).unwrap();

                    let path = dir.into_path();
                    for package in v {
                        tx.send((String::from(package), path.clone())).unwrap()
                    }
                }
            }

            Continue
        })
    });

    // Iterate over Python files in parallel in the current directory
    let types = TypesBuilder::new().add_defaults().select("py").build()?;
    let walker = WalkBuilder::new("./")
        .standard_filters(true)
        .types(types)
        .build_parallel();
    walker.run(|| {
        let tx = tx.clone();
        Box::new(move |result| {
            use ignore::WalkState::Continue;

            if let Ok(dir) = result {
                if dir.file_type().unwrap().is_file() {
                    let contents = fs::read_to_string(dir.path()).unwrap();
                    let v = parse_python_file(&contents).unwrap();

                    let path = dir.into_path();
                    for package in v {
                        tx.send((String::from(package), path.clone())).unwrap()
                    }
                }
            }

            Continue
        })
    });

    drop(tx);
    if let Err(err) = stdout_thread.join().unwrap() {
        // A broken pipe means graceful termination, so fall through.
        // Otherwise, something bad happened while writing to stdout, so bubble
        // it up.
        if err.kind() != io::ErrorKind::BrokenPipe {
            return Err(err.into());
        }
    }

    Ok(())
}
