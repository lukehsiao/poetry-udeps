use std::{
    io::{self, Write},
    thread,
};

use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use ignore::{types::TypesBuilder, WalkBuilder};
use log::info;
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

pub fn run(_cli: Cli) -> Result<()> {
    let venv_path = get_venv_path()?;
    info!("Reading files in venv: {}", venv_path);

    let (tx, rx) = crossbeam_channel::bounded::<String>(100);

    // Setup main thread for stdout
    let stdout_thread = thread::spawn(move || {
        let mut stdout = io::BufWriter::new(io::stdout());
        for entity in rx {
            stdout.write(entity.as_bytes()).unwrap();
            stdout.write(b"\n").unwrap();
        }
    });

    // Iterate over Python files in parallel in the venv
    let types = TypesBuilder::new()
        .add_defaults()
        .select("py")
        .select("jupyter")
        .build()?;
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
    let types = TypesBuilder::new()
        .add_defaults()
        .select("py")
        .select("jupyter")
        .build()?;
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
