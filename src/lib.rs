use std::{
    collections::HashSet,
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

fn get_dependencies(file: &Path) -> Result<(HashSet<String>, HashSet<String>)> {
    let toml = fs::read_to_string(file)?;

    let value = toml.parse::<Value>()?;
    let dependencies: HashSet<String> = value
        .get("tool")
        .unwrap()
        .get("poetry")
        .unwrap()
        .get("dependencies")
        .unwrap()
        .as_table()
        .unwrap()
        .keys()
        .map(|s| String::from(s))
        .collect();
    info!("Dependencies: {:#?}", dependencies);
    let dev_dependencies: HashSet<String> = value
        .get("tool")
        .unwrap()
        .get("poetry")
        .unwrap()
        .get("dev-dependencies")
        .unwrap()
        .as_table()
        .unwrap()
        .keys()
        .map(|s| String::from(s))
        .collect();
    info!("Dev Dependencies: {:#?}", dev_dependencies);
    Ok((dependencies, dev_dependencies))
}

pub fn run(_cli: Cli) -> Result<()> {
    let pyproject_path = Path::new("pyproject.toml");
    let (deps, dev_deps) = get_dependencies(&pyproject_path)?;

    let venv_path = get_venv_path()?;
    info!("Reading files in venv: {}", venv_path);

    let (tx, rx) = crossbeam_channel::bounded::<String>(100);

    // Setup main thread for stdout
    let stdout_thread = thread::spawn(move || {
        let mut stdout = io::BufWriter::new(io::stdout());
        for entity in rx {
            // stdout.write(entity.as_bytes()).unwrap();
            // stdout.write(b"\n").unwrap();
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
