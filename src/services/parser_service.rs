//! Parser service for Quake log files.

use crate::models::{Game, Kill};
use crate::services::{game_service::GameService, kill_service::KillService};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Defines the interface for parsing Quake log files.
pub trait ParserService {
    /// Parses a Quake log file and returns a vector of Game instances.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the log file to parse.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of Game instances or an IO error.
    fn parse_log(file_path: &str) -> Result<Vec<Game>, std::io::Error>;
}

/// Implements the ParserService for Quake log files.
pub struct QuakeParser;

impl ParserService for QuakeParser {
    /// Parses a Quake log file.
    ///
    /// This function reads the log file line by line, identifying game initializations,
    /// shutdowns, kills, and player information. It constructs Game instances
    /// with the parsed data.
    ///
    /// # Errors
    ///
    /// Returns an IO error if the file cannot be read or processed.
    fn parse_log(file_path: &str) -> Result<Vec<Game>, std::io::Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut games: Vec<Game> = Vec::new();
        let mut current_game: Option<Game> = None;

        let kill_regex =
            Regex::new(r"(\d+:\d+) Kill: (\d+) (\d+) (\d+): (.+) killed (.+) by (.+)").unwrap();
        let init_game_regex = Regex::new(r"InitGame:").unwrap();
        let shutdown_game_regex = Regex::new(r"ShutdownGame:").unwrap();
        let client_userinfo_changed_regex =
            Regex::new(r"ClientUserinfoChanged: \d+ n\\(.+?)\\").unwrap();

        for line in reader.lines() {
            let line = line?;

            if init_game_regex.is_match(&line) {
                if let Some(game) = current_game.take() {
                    games.push(game);
                }
                current_game = Some(Game::new());
            } else if shutdown_game_regex.is_match(&line) {
                if let Some(game) = current_game.take() {
                    games.push(game);
                }
            } else if let Some(caps) = kill_regex.captures(&line) {
                if let Some(game) = &mut current_game {
                    let killer = caps.get(5).unwrap().as_str().to_string();
                    let victim = caps.get(6).unwrap().as_str().to_string();
                    let weapon = caps.get(7).unwrap().as_str().to_string();
                    game.add_kill(Kill::new(killer, victim, weapon));
                }
            } else if let Some(caps) = client_userinfo_changed_regex.captures(&line) {
                if let Some(game) = &mut current_game {
                    let player_name = caps.get(1).unwrap().as_str().to_string();
                    game.add_player(player_name);
                }
            }
        }

        if let Some(game) = current_game {
            games.push(game);
        }

        Ok(games)
    }
}
