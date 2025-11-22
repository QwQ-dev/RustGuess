use rust_guess::config::settings_handler::SETTINGS;
use rust_guess::game_handler::{
    get_user_data_from_console, handle_input_error, print_welcome_msg, start_game,
};
use rust_guess::input::std_input_source::StdInputSource;
use rust_guess::web::web_api::start_web_api_service;

mod boxed_error;
mod data_handler;
mod game_error;
mod input;
mod user_data;

#[tokio::main]
async fn main() {
    let _ = &*SETTINGS;

    tokio::spawn(async {
        start_web_api_service().await;
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let mut std_input_source = StdInputSource::new();
    let user_data = get_user_data_from_console(&mut std_input_source).await;

    match user_data {
        Ok(mut user_data) => {
            print_welcome_msg(&user_data);
            start_game(&mut std_input_source, &mut user_data);

            if let Err(boxed_error) = user_data.save().await {
                eprintln!("Error saving user data: {}", boxed_error);
            }
        }
        Err(game_error) => {
            handle_input_error(game_error);
        }
    }
}
