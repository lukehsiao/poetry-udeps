use std::{
    collections::BTreeMap,
    fs,
    io::{self, Write},
    path::Path,
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
use crate::name_map::KNOWN_NAMES;

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
    // let sh = Shell::new()?;
    let toml = fs::read_to_string(file)?;

    // TODO: map package name to actual module name.
    // Ref: https://stackoverflow.com/a/54853084
    let value = toml.parse::<Value>()?;
    let dep_key = match deps {
        DepType::Main => "dependencies",
        DepType::Dev => "dev_dependencies",
    };
    let dependencies: BTreeMap<String, Option<String>> = value
        .get("tool")
        .unwrap()
        .get("poetry")
        .unwrap()
        .get(dep_key)
        .unwrap()
        .as_table()
        .unwrap()
        .keys()
        .map(|s| {
            let package = String::from(s);
            let alias = KNOWN_NAMES.get(&package).map(|a| String::from(*a));
            (package, alias)
        })
        .collect();
    Ok(dependencies)
}

fn get_deps_from_file(_file: &Path) -> Result<Vec<String>> {
    Ok(vec![])
}

pub fn run(_cli: Cli) -> Result<()> {
    let pyproject_path = Path::new("pyproject.toml");
    let main_deps = get_dependencies(pyproject_path, DepType::Main)?;
    info!("Main Deps: {:#?}", main_deps);
    let dev_deps = get_dependencies(pyproject_path, DepType::Dev)?;
    info!("Dev Deps: {:#?}", dev_deps);

    let venv_path = get_venv_path()?;
    info!("Reading files in venv: {}", venv_path);

    let (tx, rx) = crossbeam_channel::bounded::<String>(100);

    // Setup main thread for stdout
    let stdout_thread = thread::spawn(move || {
        let mut stdout = io::BufWriter::new(io::stdout());
        for entity in rx {
            stdout.write_all(entity.as_bytes()).unwrap();
            stdout.write_all(b"\n").unwrap();
        }
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
                    tx.send(dir.path().to_str().unwrap().to_owned()).unwrap()
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
                    tx.send(dir.path().to_str().unwrap().to_owned()).unwrap()
                }
            }

            Continue
        })
    });

    drop(tx);
    stdout_thread.join().unwrap();

    Ok(())
}
