use std::collections::HashMap;

pub struct Game {
    pub total_kills: u32,
    pub players: Vec<String>,
    pub kills: HashMap<String, i32>,
    pub kills_by_means: HashMap<String, u32>,
}
