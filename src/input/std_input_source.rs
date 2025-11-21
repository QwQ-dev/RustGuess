use crate::game_error::GameError;
use crate::input::InputSource;
use std::io;

#[allow(dead_code)]
pub struct StdInputSource;

impl StdInputSource {
    #[allow(dead_code)]
    pub fn new() -> Self {
        StdInputSource
    }
}

impl InputSource for StdInputSource {
    fn read_line(&mut self) -> Result<String, GameError> {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        let trimmed = buffer.trim();

        if trimmed == "exit" || trimmed == "quit" || trimmed == "end" {
            return Err(GameError::ExitCommand);
        }

        Ok(trimmed.to_string())
    }
}
