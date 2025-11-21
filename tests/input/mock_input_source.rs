use rust_guess::game_error::GameError;
use rust_guess::input::InputSource;

#[allow(dead_code)]
pub struct MockInputSource {
    pub inputs: std::vec::IntoIter<String>,
}

impl InputSource for MockInputSource {
    fn read_line(&mut self) -> Result<String, GameError> {
        match self.inputs.next() {
            Some(input) => {
                let processed_input = input.trim().to_lowercase();

                match processed_input.as_str() {
                    "exit" | "quit" | "end" => Err(GameError::ExitCommand),
                    _ => Ok(input),
                }
            }
            None => Err(GameError::ExitCommand),
        }
    }
}
