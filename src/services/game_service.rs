use crate::models::{Game, Kill};
use std::collections::{HashMap, HashSet};

pub trait GameService {
    fn new() -> Self;
    fn add_kill(&mut self, kill: Kill);
    fn add_player(&mut self, player: String);
}

impl GameService for Game {
    fn new() -> Self {
        Game {
            total_kills: 0,
            players: HashSet::new(),
            kills: HashMap::new(),
            kills_by_means: HashMap::new(),
        }
    }

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

    fn add_player(&mut self, player: String) {
        if player != "<world>" && self.players.insert(player.clone()) {
            self.kills.insert(player, 0);
        }
    }
}
