use crate::game_error::GameError;

#[allow(dead_code)]
pub trait InputSource {
    fn read_line(&mut self) -> Result<String, GameError>;
}
