//! Contains a single function to build the CLI
use clap::{Arg, ArgAction, ArgMatches, Command};

/// Builds the CLI so the main file doesn't get cluttered.
pub fn build() -> ArgMatches {
    Command::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        // .author(clap::crate_authors!("\n"))
        .long_about("This program will do something.")
        .arg(
            Arg::new("read")
                .value_name("FILE(S)")
                .help("One or more file(s) to process. Wildcards and multiple_occurrences files (e.g. 2019*.pdf 2020*.pdf) are supported.")
                .required(true)
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
        .arg( // Don't export detail information
            Arg::new("detail-off")
                .short('o')
                .long("detail-off")
                .help("Don't export detailed information about each file processed.")
                .num_args(0)
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("csv-filename")
                .short('c')
                .long("csv-filename")
                .help("The name of the resulting CSV file containing the video metadata summaries.")
                .num_args(..=1)
                .default_missing_value("video-data.csv")
                .action(ArgAction::Set)
        )
        .get_matches()
}
