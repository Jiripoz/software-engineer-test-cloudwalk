use crate::models::{Game, Kill};
use crate::services::{game_service::GameService, kill_service::KillService};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub trait ParserService {
    fn parse_log(file_path: &str) -> Result<Vec<Game>, std::io::Error>;
}

pub struct QuakeParser;

impl ParserService for QuakeParser {
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
