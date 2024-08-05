//! Report generation services for Quake game statistics.

use std::collections::HashMap;
use crate::models::Game;

/// Defines the interface for generating reports from game data.
pub trait ReportService {
    /// Prints a detailed report for each game.
    ///
    /// # Arguments
    ///
    /// * `games` - A slice of Game instances to report on.
    fn print_report(games: &[Game]);
    /// Prints a ranking of players based on their total kills across all games.
    ///
    /// # Arguments
    ///
    /// * `games` - A slice of Game instances to calculate the ranking from.
    fn print_player_ranking(games: &[Game]);
}


/// Implements the ReportService for Quake game reports.
pub struct QuakeReport;

impl ReportService for QuakeReport {
    /// Prints a detailed report for each game and an overall player ranking.
    ///
    /// This function displays:
    /// - Total kills per game
    /// - Players in each game
    /// - Kill count per player in each game
    /// - Kills by weapon type in each game
    /// - Overall player ranking
    fn print_report(games: &[Game]) {
        for (i, game) in games.iter().enumerate() {
            println!("Game {}:", i + 1);
            println!("  Total kills: {}", game.total_kills);
            println!("  Players: {:?}", game.players);
            println!("  Kills:");
            for (player, kills) in &game.kills {
                println!("    {}: {}", player, kills);
            }
            println!("  Kills by means:");
            for (weapon, kills) in &game.kills_by_means {
                println!("    {}: {}", weapon, kills);
            }
            println!();
        }
    
        Self::print_player_ranking(games);
    }    

    /// Calculates and prints the overall player ranking based on total kills.
    ///
    /// This function:
    /// - Aggregates kills for each player across all games
    /// - Sorts players by their total kill count
    /// - Prints the sorted ranking
    fn print_player_ranking(games: &[Game]) {
        let mut total_kills: HashMap<String, i32> = HashMap::new();
    
        for game in games {
            for (player, kills) in &game.kills {
                *total_kills.entry(player.clone()).or_insert(0) += kills;
            }
        }
    
        let mut ranking: Vec<(String, i32)> = total_kills.into_iter().collect();
        ranking.sort_by(|a, b| b.1.cmp(&a.1));
    
        println!("Player Ranking:");
        for (i, (player, kills)) in ranking.iter().enumerate() {
            println!("  {}. {}: {}", i + 1, player, kills);
        }
    }
}
