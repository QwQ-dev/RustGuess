use crate::boxed_error::BoxedError;
use crate::user_data::UserData;
use std::path::PathBuf;
use tokio::fs;

#[allow(dead_code)]
pub fn get_user_path_buf(user_name: &str) -> PathBuf {
    PathBuf::from("users")
        .join(user_name)
        .with_extension("json")
}

#[allow(dead_code)]
pub async fn save_user_data(user_data: &UserData) -> Result<(), BoxedError> {
    let user_name = user_data.get_name();
    let user_data_string = serde_json::to_string(&user_data).map_err(BoxedError::from)?;
    let user_path_buf = get_user_path_buf(user_name);

    if let Some(parent_dir) = user_path_buf.parent() {
        fs::create_dir_all(parent_dir)
            .await
            .map_err(BoxedError::from)?;
    }

    Ok(fs::write(user_path_buf, user_data_string).await?)
}

#[allow(dead_code)]
pub async fn get_user_data_from_file(user_name: &str) -> Result<UserData, BoxedError> {
    let user_path = get_user_path_buf(user_name);

    if user_path.exists() {
        let user_data_string = fs::read_to_string(user_path).await?;
        return serde_json::from_str(&user_data_string).map_err(BoxedError::from);
    }

    Err(BoxedError::from(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "User not found",
    )))
}

#[allow(dead_code)]
pub async fn delete_user_data_from_file(user_name: &str) -> Result<(), BoxedError> {
    let path_buf = get_user_path_buf(user_name);
    let path = path_buf.as_path();

    match fs::remove_file(path).await {
        Ok(_) => {}
        Err(err) => {
            // if not found, it's ok
            if err.kind() == std::io::ErrorKind::NotFound {
                return Ok(());
            }
            return Err(BoxedError::from(err));
        }
    }

    if let Some(parent_dir) = path_buf.parent() {
        let _ = fs::remove_dir(parent_dir).await;
    }

    Ok(())
}
