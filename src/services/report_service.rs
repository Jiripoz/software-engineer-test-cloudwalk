use crate::models::Game;

pub trait ReportService {
    fn print_report(games: &[Game]);
    fn print_player_ranking(games: &[Game]);
}
