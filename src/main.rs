//! # Quake Log Parser
//!
//! This is the main entry point for the Quake Log Parser application.
//! It reads a log file, parses the games, and generates a report.

#[cfg(test)]
pub mod tests;

pub mod models;
pub mod services;

use services::{ParserService, QuakeParser};
use services::{ReportService, QuakeReport};

/// The main function of the Quake Log Parser application.
///
/// # Error Handling
///
/// This function will exit with a status code of 1 if:
/// - No file path is provided as a command-line argument.
/// - The log file cannot be parsed successfully.
///
/// # Example
///
/// ```shell
/// $ cargo run -- path/to/logfile.log
/// ```
fn main() {
    let file_path = get_file_path();
    let games = parse_games(&file_path);
    print_report(games);
}

/// Retrieves the file path from command-line arguments.
///
/// # Panics
///
/// This function will cause the program to exit if no file path is provided.
fn get_file_path() -> String {
    std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please provide the path to the log file as an argument.");
        std::process::exit(1);
    })
}

/// Parses the games from the given log file.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the log file
///
/// # Panics
///
/// This function will cause the program to exit if the log file cannot be parsed.
fn parse_games(file_path: &str) -> Vec<models::Game> {
    QuakeParser::parse_log(file_path).unwrap_or_else(|err| {
        eprintln!("Failed to parse log file: {:?}", err);
        std::process::exit(1);
    })
}

/// Prints the report for the parsed games.
///
/// # Arguments
///
/// * `games` - A vector of `Game` structs representing the parsed games
fn print_report(games: Vec<models::Game>) {
    QuakeReport::print_report(&games);
}