//! Contains a single function to build the CLI
use clap::{Arg, ArgAction, ArgMatches, Command};

/// Builds the CLI so the main file doesn't get cluttered.
///
/// # Arguments
///
/// None.
///
/// # Returns
///
/// * `ArgMatches` - The command line arguments
pub fn build() -> ArgMatches {
    Command::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        // .author(clap::crate_authors!("\n"))
        .long_about("Extracts information from MP4 files into a CSV.")
        .arg(
            Arg::new("read")
                .value_name("FILE(S)")
                .help("One or more file(s) to process. Wildcards and multiple occurrences (e.g. here/*.mp4 there/*.MP4) are supported.")
                .required(false)
                .num_args(1..)
                .action(ArgAction::Append)
        )
        .arg( // Hidden debug parameter
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Output debug information as we go. Supply it twice for trace-level logs.")
                .hide(true)
                .env("MP4CSV_DEBUG")
                .num_args(0)
                .action(ArgAction::Count)
        )
        .arg( // Don't print any information
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Don't produce any output except errors while working.")
                .num_args(0)
                .action(ArgAction::SetTrue)
        )
        .arg( // Print summary information
            Arg::new("print-summary")
                .short('p')
                .long("print-summary")
                .help("Print summary detail for each session processed.")
                .num_args(0)
                .action(ArgAction::SetTrue)
        )
        .arg( // Input file - a CSV with a list of files as the first column
            Arg::new("input-csv")
                .short('i')
                .long("input-csv")
                .help("A CSV file with a list of files to process in the first column. The first row is assumed to be a header and is ignored.")
                .num_args(1)
                .action(ArgAction::Set)
                .conflicts_with("read")
        )
        .arg(
            Arg::new("csv-filename")
                .short('c')
                .long("csv-filename")
                .help("The name of the resulting CSV file containing the video metadata summaries. Default is `video-data.csv` is none is specified.")
                .num_args(..=1)
                .default_missing_value("video-data.csv")
                .action(ArgAction::Set)
        )
        .get_matches()
}
