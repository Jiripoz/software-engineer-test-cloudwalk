//! Game-related services and implementations.

use crate::models::{Game, Kill};
use std::collections::{HashMap, HashSet};

/// Defines the core functionality for managing game state.
pub trait GameService {
    /// Creates a new instance of the game.
    fn new() -> Self;

    /// Adds a kill to the game state.
    ///
    /// # Arguments
    ///
    /// * `kill` - The Kill struct containing information about the kill.
    fn add_kill(&mut self, kill: Kill);

    /// Adds a player to the game.
    ///
    /// # Arguments
    ///
    /// * `player` - The name of the player to add.
    fn add_player(&mut self, player: String);
}

impl GameService for Game {
    /// Creates a new Game instance with initialized fields.
    ///
    /// # Returns
    ///
    /// A new Game struct with empty fields.
    fn new() -> Self {
        Game {
            total_kills: 0,
            players: HashSet::new(),
            kills: HashMap::new(),
            kills_by_means: HashMap::new(),
        }
    }

    /// Processes a kill, updating the game accordingly.
    ///
    /// This method updates total kills, player list, kill counts, and kill means.
    ///
    /// # Arguments
    ///
    /// * `kill` - The Kill struct containing information about the kill.
    ///
    /// # Note
    ///
    /// If the killer is \<world\>, the victim's kill count is decremented.
    fn add_kill(&mut self, kill: Kill) {
        self.total_kills += 1;

        self.add_player(kill.victim.clone());
        if kill.killer != "<world>" {
            self.add_player(kill.killer.clone());
        }

        if kill.killer == "<world>" {
            *self.kills.entry(kill.victim).or_insert(0) -= 1;
        } else {
            *self.kills.entry(kill.killer).or_insert(0) += 1;
        }

        *self.kills_by_means.entry(kill.weapon).or_insert(0) += 1;
    }

    /// Adds a player to the game if they don't already exist.
    ///
    /// This method also initializes the player's kill count to 0.
    ///
    /// # Arguments
    ///
    /// * `player` - The name of the player to add.
    ///
    /// # Note
    ///
    /// The \<world\> entity is not considered a player and will not be added.
    fn add_player(&mut self, player: String) {
        if player != "<world>" && self.players.insert(player.clone()) {
            self.kills.insert(player, 0);
        }
    }
}