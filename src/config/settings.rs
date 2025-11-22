use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub min_number: i8,
    pub max_number: i8,
    pub web_api_address: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            min_number: 0,
            max_number: 100,
            web_api_address: "0.0.0.0:3000".to_string(),
        }
    }
}
