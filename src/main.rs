#[cfg(test)]
pub mod tests;

pub mod models;
pub mod services;

use services::{ParserService, QuakeParser};
use services::{ReportService, QuakeReport};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = args.get(1).unwrap_or_else(|| {
        eprintln!("Please provide the path to the log file as an argument.");
        std::process::exit(1);
    });
    let games_result = QuakeParser::parse_log(file_path);
    if games_result.is_err() {
        eprintln!("Failed to parse log file: {:?}", games_result.err());
        std::process::exit(1);
    }
    let games = games_result.unwrap();
    QuakeReport::print_report(&games);
}
