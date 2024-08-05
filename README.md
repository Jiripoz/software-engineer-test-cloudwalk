# Quake Log Parser - Software Engineer Test

This project parses Quake 3 Arena server log files and generates reports for each game, including kill statistics, player rankings, and death causes.

## Features

- Read and parse Quake 3 Arena log files
- Group game data for each match
- Collect and analyze kill data
- Generate reports for each match
- Create a player ranking
- Provide death statistics grouped by cause

## Prerequisites

* [Rust 1.80.0](https://www.rust-lang.org/)

## Installation

1. Clone the repository
2. Build the project: `cargo build --release`

## Usage

Run the program with the default log file:
`cargo run -- data/qgames.log`

Or use a custom log file: 
`cargo run -- path/to/your/logfile.log`

## Testing

Run unit tests with:
`cargo test`

## Documentation

The code is documented using Rust's built-in documentation system. Generate and view the documentation:
`cargo doc --open`

## About me 

* Name: Alan Franco de Oliveira
* LinkedIn: [@alanfranco7](https://www.linkedin.com/in/alanfranco7/)
* Email: alan.franco.7@gmail.com or contact@jiripo.dev
