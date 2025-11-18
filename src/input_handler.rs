use crate::game_error::GameError;
use std::io;

pub fn get_input() -> Result<String, GameError> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let trimmed = buffer.trim();

    if trimmed == "exit" || trimmed == "quit" || trimmed == "end" {
        return Err(GameError::ExitCommand);
    }

    Ok(trimmed.to_string())
}
