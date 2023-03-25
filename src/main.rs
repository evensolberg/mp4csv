use clap::parser::ValueSource; // Command line
use glob::glob;
use std::error::Error;

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

mod cli;

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

    // Start processing stuff and things
    for argument in cli_args
        .get_many::<String>("read")
        .unwrap_or_default()
        .map(std::string::String::as_str)
    {
        for entry in glob(&argument).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => println!("{argument} -- {:?}", path.display()),
                Err(e) => println!("{:?}", e),
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
            log::error!("{}", err.to_string().replace("\"", ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
