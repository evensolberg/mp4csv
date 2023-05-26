use clap::parser::ValueSource; // Command line

use std::error::Error;

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

mod cli;
mod parser;
mod utils;

use parser::VideoInfo;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = cli::build();

    // create a log builder
    let mut logbuilder = Builder::new();

    // Figure out what log level to use.
    if cli_args.value_source("quiet") == Some(ValueSource::CommandLine) {
        logbuilder.filter_level(LevelFilter::Off);
    } else {
        match cli_args.get_count("debug") {
            0 => logbuilder.filter_level(LevelFilter::Info),
            1 => logbuilder.filter_level(LevelFilter::Debug),
            _ => logbuilder.filter_level(LevelFilter::Trace),
        };
    }

    // Initialize logging
    logbuilder.target(Target::Stdout).init();

    // Set some flags to determine how to behave
    let quiet = cli_args.value_source("quiet") == Some(ValueSource::CommandLine);
    let print_summary = cli_args.value_source("print-summary") == Some(ValueSource::CommandLine);

    // Create a vector to store the video info structs
    let mut video_info: Vec<VideoInfo> = Vec::new();

    // Start processing stuff and things
    let mut files_processed = 0;
    for filename in cli_args
        .get_many::<String>("read")
        .unwrap_or_default()
        .map(std::string::String::as_str)
    {
        if !quiet {
            log::info!("Processing: {filename}");
        }

        let vi = VideoInfo::from(filename)?;
        log::debug!("vi = {vi:#?}");
        video_info.push(vi);
        files_processed += 1;
    }

    // Write the summary CSV file
    let default_filename = "video-data.csv".to_string();
    let csv_filename = cli_args
        .get_one::<String>("csv-filename")
        .unwrap_or(&default_filename);
    export_csv(&video_info, csv_filename)?;

    if !quiet && print_summary {
        log::info!("Files processed: {files_processed}");
    }

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

fn export_csv(vi: &Vec<VideoInfo>, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(filename)?;
    for v in vi {
        wtr.serialize(v)?;
    }
    wtr.flush()?;
    Ok(())
}
