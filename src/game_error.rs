use crate::boxed_error::BoxedError;

#[derive(Debug)]
#[allow(dead_code)]
pub enum GameError {
    Error(BoxedError),
    ExitCommand,
}

impl From<std::io::Error> for GameError {
    fn from(error: std::io::Error) -> Self {
        GameError::Error(Box::new(error))
    }
}

impl PartialEq for GameError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (GameError::ExitCommand, GameError::ExitCommand)
        )
    }
}
