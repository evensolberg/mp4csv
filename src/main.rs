use clap::parser::ValueSource; // Command line
use glob::glob; // File globbing

use std::error::Error;

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

mod cli;
mod inout;
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
    let export_to_csv = cli_args.value_source("csv-filename") == Some(ValueSource::CommandLine);
    let export_to_json = cli_args.value_source("json-filename") == Some(ValueSource::CommandLine);

    if !export_to_csv && !export_to_json {
        log::error!("You must specify either --csv-filename or --json-filename");
        std::process::exit(1);
    }

    let files_to_process = inout::files_to_process(&cli_args)?;

    // Start processing stuff and things
    let mut video_info: Vec<VideoInfo> = Vec::new();

    let mut filenames: Vec<String> = Vec::new();

    // Expand the paths
    extract_paths(files_to_process, &mut filenames)?;

    // Process the files
    let files_processed = process_files(filenames, quiet, &mut video_info)?;

    // Write the summary CSV file
    write_csv(export_to_csv, &cli_args, quiet, &video_info)?;

    // Write the summary JSON file
    write_json(export_to_json, cli_args, quiet, video_info)?;

    if !quiet && print_summary {
        log::info!("Files processed: {files_processed}");
    }

    Ok(())
}

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

/// Writes the video info to a CSV file
///
/// # Arguments
///
/// * `vi` - A reference to a vector of `VideoInfo` structs
/// * `filename` - The name of the CSV file to write
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - An empty result if everything went well, or an error if not.
fn export_csv(vi: &Vec<VideoInfo>, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(filename)?;
    for v in vi {
        wtr.serialize(v)?;
    }
    wtr.flush()?;
    Ok(())
}

/// Writes the video info to a CSV file
///
/// # Arguments
///
/// * `vi` - A reference to a vector of `VideoInfo` structs
/// * `filename` - The name of the CSV file to write
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - An empty result if everything went well, or an error if not.
fn export_json(vi: &Vec<VideoInfo>, filename: &str) -> Result<(), Box<dyn Error>> {
    Ok(serde_json::to_writer_pretty(
        std::fs::File::create(filename)?,
        vi,
    )?)
}

/// Decide whether to write to JSON or not. If so, write it.
fn write_json(
    export_to_json: bool,
    cli_args: clap::ArgMatches,
    quiet: bool,
    video_info: Vec<VideoInfo>,
) -> Result<(), Box<dyn Error>> {
    if export_to_json {
        let json_filename = inout::output_json_filename(&cli_args);

        if !quiet {
            log::info!("Writing summary to JSON file: {json_filename}");
        }

        export_json(&video_info, json_filename.as_str())?;
    };
    Ok(())
}

/// Decide whether to write to CSV or not. If so, write it.
fn write_csv(
    export_to_csv: bool,
    cli_args: &clap::ArgMatches,
    quiet: bool,
    video_info: &Vec<VideoInfo>,
) -> Result<(), Box<dyn Error>> {
    if export_to_csv {
        let csv_filename = inout::output_csv_filename(cli_args);

        if !quiet {
            log::info!("Writing summary to CSV file: {csv_filename}");
        }

        export_csv(video_info, csv_filename.as_str())?;
    };
    Ok(())
}

/// Expand the paths in the command line arguments
fn extract_paths(
    files_to_process: Vec<String>,
    filenames: &mut Vec<String>,
) -> Result<(), Box<dyn Error>> {
    for input_path in files_to_process {
        let glob = glob(input_path.as_str())?;
        log::debug!("glob = {glob:?}");
        for entry in glob {
            match entry {
                Ok(path) => {
                    filenames.push(path.to_str().unwrap().to_string());
                }
                Err(e) => {
                    log::error!("Error processing glob: {}", e);
                }
            }
        }
    }
    Ok(())
}

fn process_files(
    filenames: Vec<String>,
    quiet: bool,
    video_info: &mut Vec<VideoInfo>,
) -> Result<u32, Box<dyn Error>> {
    let mut files_processed = 0;
    for filename in &filenames {
        if !quiet {
            log::info!("Processing: {filename}");
        }

        let vi = VideoInfo::from(filename.as_str())?;
        log::debug!("vi = {vi:#?}");
        video_info.push(vi);
        files_processed += 1;
    }
    Ok(files_processed)
} // fn run()
