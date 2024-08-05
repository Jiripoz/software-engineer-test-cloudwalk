use std::collections::HashSet;

use crate::models::{Game, Kill};
use crate::services::{GameService, KillService};

#[test]
fn test_game_creation() {
    let game = Game::new();
    assert_eq!(game.total_kills, 0);
    assert!(game.players.is_empty());
    assert!(game.kills.is_empty());
    assert!(game.kills_by_means.is_empty());
}

#[test]
fn test_add_kill() {
    let mut game = Game::new();
    game.add_kill(Kill::new(
        "Player1".to_string(),
        "Player2".to_string(),
        "MOD_RAILGUN".to_string(),
    ));

    let expected_players: HashSet<String> = ["Player1".to_string(), "Player2".to_string()]
        .into_iter()
        .collect();
    assert_eq!(game.players, expected_players);
    assert_eq!(game.players.len(), 2);

    assert_eq!(game.kills.get("Player1"), Some(&1));
    assert_eq!(game.kills.get("Player2"), None);
    assert_eq!(game.kills_by_means.get("MOD_RAILGUN"), Some(&1));
}

#[test]
fn test_add_player() {
    let mut game = Game::new();

    // Add a new player
    game.add_player("Player1".to_string());
    assert_eq!(game.total_kills, 0);
    assert_eq!(game.players, HashSet::from(["Player1".to_string()]));
    assert!(game.kills.is_empty());
    assert!(game.kills_by_means.is_empty());

    // Add the same player again
    game.add_player("Player1".to_string());
    assert_eq!(game.players.len(), 1); // Should still be 1

    // Add another player
    game.add_player("Player2".to_string());
    assert_eq!(game.players.len(), 2);

    // Try to add "<world>" as a player
    game.add_player("<world>".to_string());
    assert_eq!(game.players.len(), 2); // Should still be 2

    // Verify final state
    let expected_players: HashSet<String> = ["Player1".to_string(), "Player2".to_string()]
        .into_iter()
        .collect();
    assert_eq!(game.players, expected_players);
    assert_eq!(game.total_kills, 0);
    assert!(game.kills.is_empty());
    assert!(game.kills_by_means.is_empty());
}
