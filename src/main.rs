use clap::parser::ValueSource; // Command line
use glob::glob;
use std::error::Error;

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

mod cli;
mod parser;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = cli::build();

    // create a log builder
    let mut logbuilder = Builder::new();

    // Figure out what log level to use.
    if cli_args.value_source("quiet") != Some(ValueSource::CommandLine) {
        match cli_args.get_count("debug") {
            0 => logbuilder.filter_level(LevelFilter::Info),
            1 => logbuilder.filter_level(LevelFilter::Debug),
            _ => logbuilder.filter_level(LevelFilter::Trace),
        };
    } else {
        logbuilder.filter_level(LevelFilter::Off);
    }

    // Initialize logging
    logbuilder.target(Target::Stdout).init();

    // Set some flags to determine how to behave
    let print_detail = cli_args.value_source("detail-off") != Some(ValueSource::CommandLine);

    // Start processing stuff and things
    for argument in cli_args
        .get_many::<String>("read")
        .unwrap_or_default()
        .map(std::string::String::as_str)
    {
        for entry in glob(&argument).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    log::debug!("{argument} -- {:?}", path.display());
                    parser::parse(&path, print_detail)?;
                }
                Err(e) => {
                    return Err(format!("Globbing failed. Error message: {e}").into());
                }
            }
        }
    }

    // Everything is a-okay in the end
    Ok(())
} // fn run()

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// The actual executable function that gets called when the program in invoked.
fn main() {
    std::process::exit(match run() {
        Ok(_) => 0, // everying is hunky dory - exit with code 0 (success)
        Err(err) => {
            log::error!("{}", err.to_string().replace('\"', ""));
            1 // exitErr(e.into()) with a non-zero return code, indicating a problem
        }
    });
}
