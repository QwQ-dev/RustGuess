use crate::boxed_error::BoxedError;

pub enum GameError {
    Error(BoxedError),
    ExitCommand,
}

impl From<std::io::Error> for GameError {
    fn from(error: std::io::Error) -> Self {
        GameError::Error(Box::new(error))
    }
}
