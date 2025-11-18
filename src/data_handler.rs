use crate::boxed_error::BoxedError;
use crate::user_data::UserData;
use std::fs;
use std::path::{Path, PathBuf};

pub fn save_user_data(user_data: UserData) -> Result<(), BoxedError> {
    let user_name = user_data.get_name();
    let user_data_string = serde_json::to_string(&user_data).map_err(BoxedError::from)?;

    let path_buf = PathBuf::from("users")
        .join(user_name)
        .with_extension("json");
    let path = Path::new(path_buf.as_path());

    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir).map_err(BoxedError::from)?;
    }

    Ok(fs::write(path, user_data_string)?)
}

pub fn get_user_data_from_file(user_name: &str) -> Result<UserData, BoxedError> {
    let path_buf = PathBuf::from("users")
        .join(user_name)
        .with_extension("json");
    let path = Path::new(path_buf.as_path());

    if path.exists() {
        let user_data_string = fs::read_to_string(path)?;
        return serde_json::from_str(&user_data_string).map_err(BoxedError::from);
    }

    Err(BoxedError::from(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "User not found",
    )))
}
