use crate::boxed_error::BoxedError;
use crate::data_handler::{get_user_data_from_file, save_user_data};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData {
    name: String,
    pub wins: i8,
    pub losses: i8,
}

impl UserData {
    #[allow(dead_code)]
    pub fn new(name: String, wins: i8, losses: i8) -> UserData {
        UserData { name, wins, losses }
    }

    #[allow(dead_code)]
    pub fn with_name(name: String) -> UserData {
        UserData {
            name,
            wins: 0,
            losses: 0,
        }
    }

    #[allow(dead_code)]
    pub async fn from_file(user_name: &str) -> Result<UserData, BoxedError> {
        get_user_data_from_file(user_name).await
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[allow(dead_code)]
    pub fn record_win(&mut self) {
        self.wins += 1;
    }

    #[allow(dead_code)]
    pub fn record_losses(&mut self) {
        self.losses += 1;
    }

    #[allow(dead_code)]
    pub async fn save(&self) -> Result<(), BoxedError> {
        save_user_data(self).await
    }
}

impl Display for UserData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User: {} | ", self.name)?;
        write!(f, "Stats: {}W / {}L", self.wins, self.losses)
    }
}
