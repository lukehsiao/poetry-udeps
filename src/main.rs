use clap::Parser;
use poetry_udeps::{run, Cli};
use std::process;

fn main() {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_max_level(convert_filter(cli.verbose.log_level_filter()))
        .init();

    match run(cli) {
        Ok(Some(deps)) => {
            for dep in deps {
                println!("{}", dep);
            }
            process::exit(1);
        }
        Ok(None) => process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(2)
        }
    }
}

fn convert_filter(filter: log::LevelFilter) -> tracing_subscriber::filter::LevelFilter {
    match filter {
        log::LevelFilter::Off => tracing_subscriber::filter::LevelFilter::OFF,
        log::LevelFilter::Error => tracing_subscriber::filter::LevelFilter::ERROR,
        log::LevelFilter::Warn => tracing_subscriber::filter::LevelFilter::WARN,
        log::LevelFilter::Info => tracing_subscriber::filter::LevelFilter::INFO,
        log::LevelFilter::Debug => tracing_subscriber::filter::LevelFilter::DEBUG,
        log::LevelFilter::Trace => tracing_subscriber::filter::LevelFilter::TRACE,
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn verify_app() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
