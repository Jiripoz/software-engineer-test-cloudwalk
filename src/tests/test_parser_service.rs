use crate::services::{ParserService, QuakeParser};
use std::collections::HashSet;

#[test]
fn test_parse_log() {
    let games_result = QuakeParser::parse_log("data/test_parse.log");
    assert!(games_result.is_ok(), "Failed to parse log file");
    
    let games = games_result.unwrap();
    assert_eq!(games.len(), 2);

    // Test first game
    let game1 = &games[0];
    assert_eq!(game1.total_kills, 6);
    
    let expected_players1: HashSet<String> = ["Isgalamido", "Dono da Bola", "Zeh"]
        .iter().map(|&s| s.to_string()).collect();
    assert_eq!(game1.players, expected_players1);
    
    assert_eq!(game1.kills.get("Isgalamido"), Some(&1));
    assert_eq!(game1.kills.get("Dono da Bola"), Some(&2));
    assert_eq!(game1.kills.get("Zeh"), Some(&1));
    assert_eq!(game1.kills_by_means.get("MOD_ROCKET_SPLASH"), Some(&4));
    assert_eq!(game1.kills_by_means.get("MOD_TRIGGER_HURT"), Some(&1));
    assert_eq!(game1.kills_by_means.get("MOD_RAILGUN"), Some(&1));

    // Test second game
    let game2 = &games[1];
    assert_eq!(game2.total_kills, 5);
    
    let expected_players2: HashSet<String> = ["Isgalamido", "Mocinha", "Zeh"]
        .iter().map(|&s| s.to_string()).collect();
    assert_eq!(game2.players, expected_players2);
    
    assert_eq!(game2.kills.get("Isgalamido"), Some(&2));
    assert_eq!(game2.kills.get("Mocinha"), Some(&1));
    assert_eq!(game2.kills.get("Zeh"), Some(&0));
    assert_eq!(game2.kills_by_means.get("MOD_ROCKET_SPLASH"), Some(&3));
    assert_eq!(game2.kills_by_means.get("MOD_ROCKET"), Some(&1));
    assert_eq!(game2.kills_by_means.get("MOD_FALLING"), Some(&1));
}
