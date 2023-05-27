//! Contains a single function to read the input CSV file

use std::error::Error;

/// Reads the provided CSV file and returns a vector of file names
///
/// # Arguments
///
/// * `filename` - A string slice that holds the name of the CSV file to read
///
/// # Returns
///
/// * `Vec<String>` - A vector of strings, each string being a file name
///
/// # Example
///
/// ```ignore
/// let files = source_csv("video-data.csv");
/// ```
fn source_csv(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(filename)?;
    let mut files: Vec<String> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        files.push(record[0].to_string());
    }

    Ok(files)
}

/// Returns a vector of file names from the command line arguments
pub fn files_to_process(cli_args: &clap::ArgMatches) -> Result<Vec<String>, Box<dyn Error>> {
    let res = if cli_args.value_source("input-csv") == Some(clap::parser::ValueSource::CommandLine)
    {
        let default = String::from("video-data.csv");
        let input_filename = cli_args.get_one::<String>("input-csv").unwrap_or(&default);
        source_csv(input_filename)?
    } else {
        let files = cli_args
            .get_many::<String>("read")
            .unwrap_or_default()
            .map(std::string::String::as_str);

        let mut names: Vec<String> = Vec::new();
        for filename in files {
            names.push(filename.to_string());
        }
        names
    };

    Ok(res)
}

/// Returns the name of the CSV file to write
pub fn output_csv_filename(cli_args: &clap::ArgMatches) -> String {
    let default_filename = "video-data.csv".to_string();
    let csv_filename = cli_args
        .get_one::<String>("csv-filename")
        .unwrap_or(&default_filename);
    csv_filename.to_string()
}
