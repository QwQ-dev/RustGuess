pub struct UserDataCleanupGuard {
    pub(crate) user_name: String,
}

impl Drop for UserDataCleanupGuard {
    fn drop(&mut self) {
        let user_name = self.user_name.clone();
        let user_name_clone = user_name.clone();

        let handle = std::thread::spawn(move || {
            let runtime = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime,
                Err(err) => {
                    eprintln!(
                        "Failed to create cleanup runtime in new thread for user {}: {:?}",
                        &user_name, err
                    );
                    return;
                }
            };

            if let Err(err) = runtime.block_on(
                rust_guess::data_handler::delete_user_data_from_file(&user_name),
            ) {
                eprintln!("Cleanup failed for user {}: {:?}", &user_name, err);
            }
        });

        if let Err(err) = handle.join() {
            eprintln!(
                "Cleanup thread join failed for user {}: {:?}",
                &user_name_clone, err
            );
        }
    }
}
