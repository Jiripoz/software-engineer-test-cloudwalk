use crate::models::Kill;

pub trait GameService {
    fn new() -> Self;
    fn add_kill(&mut self, kill: Kill);
}
