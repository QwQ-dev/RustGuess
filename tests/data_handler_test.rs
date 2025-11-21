use crate::cleanup::user_data_cleanup_guard::UserDataCleanupGuard;
use rust_guess::data_handler::{delete_user_data_from_file, get_user_path_buf};
use rust_guess::user_data::UserData;
use std::fs;
use uuid::Uuid;

mod cleanup;
mod input;
mod web;

#[tokio::test]
async fn test() {
    let user_data = UserData::with_name(Uuid::new_v4().to_string());
    let user_name = user_data.get_name();
    let user_name_string = user_name.to_string().to_owned();
    let user_path_buf = get_user_path_buf(user_name);

    let _guard = UserDataCleanupGuard {
        user_name: user_name_string,
    };

    user_data.save().await.expect("Error saving user data");

    let file_exists = fs::exists(&user_path_buf).expect("Error checking file existence");

    assert!(
        file_exists,
        "File was not created after successful save. Path: {:?}",
        &user_path_buf
    );

    delete_user_data_from_file(user_name)
        .await
        .expect("Error deleting user data");
}
