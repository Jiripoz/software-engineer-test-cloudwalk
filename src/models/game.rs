//! Defines the core game data structure for Quake log parsing.

use std::collections::{HashMap, HashSet};

/// Represents a single Quake game session with its statistics.
pub struct Game {
    /// Total number of kills in the game.
    pub total_kills: u32,

    /// Set of unique player names who participated in the game.
    pub players: HashSet<String>,

    /// Map of player names to their kill counts.
    /// Note: Kill counts can be negative if a player is killed by the world more than they kill others.
    pub kills: HashMap<String, i32>,

    /// Map of weapon/death cause to the number of kills by that means.
    pub kills_by_means: HashMap<String, u32>,
}