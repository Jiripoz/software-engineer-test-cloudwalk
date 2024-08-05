use crate::models::Kill;

pub trait KillService {
    fn new(killer: String, victim: String, weapon: String) -> Self;
}

impl KillService for Kill {
    fn new(killer: String, victim: String, weapon: String) -> Self {
        Kill {
            killer,
            victim,
            weapon,
        }
    }
}