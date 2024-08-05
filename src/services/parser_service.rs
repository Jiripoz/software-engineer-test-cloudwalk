use crate::models::Game;

pub trait ParserService {
    fn parse_log(file_path: &str) -> Result<Vec<Game>, std::io::Error>;
}
