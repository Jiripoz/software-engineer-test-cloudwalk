use std::collections::HashMap;

use crate::models::Game;

pub trait ReportService {
    fn print_report(games: &[Game]);
    fn print_player_ranking(games: &[Game]);
}

pub struct QuakeReport;

impl ReportService for QuakeReport {
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
