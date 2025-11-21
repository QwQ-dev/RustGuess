use crate::boxed_error::get_concrete_error;
use crate::game_error::GameError;
use crate::input::InputSource;
use crate::user_data::UserData;
use rand::Rng;
use std::cmp::Ordering;
use std::io::ErrorKind;

/// Handles errors originating from input operations.
///
/// This function prints an appropriate message and determines the program's
/// next control flow action (continue loop or exit function).
///
/// # Arguments
///
/// * `game_error` - The specific error that occurred.
///
/// # Returns
///
/// * `bool` - Returns `true` if the program should continue the loop (i.e., the error was handled, and the user should re-enter input).
///   Returns `false` if the program should exit the current function (i.e., a termination command was received).
///
/// # Panics
///
/// This function does not panic.
pub fn handle_input_error(game_error: GameError) -> bool {
    match game_error {
        GameError::ExitCommand => {
            println!("Goodbye!");
            false
        }
        GameError::Error(err) => {
            println!("{}", err);
            true
        }
    }
}

pub async fn get_user_data_from_console<I: InputSource>(
    input_source: &mut I,
) -> Result<UserData, GameError> {
    println!("Enter the user name: ");

    let user_name = input_source.read_line()?;
    match UserData::from_file(&user_name).await {
        Ok(user_data) => Ok(user_data),
        Err(box_error) => {
            if let Some(io_err) = get_concrete_error::<std::io::Error>(&box_error)
                && io_err.kind() == ErrorKind::NotFound
            {
                return Ok(UserData::with_name(user_name));
            }
            Err(GameError::Error(box_error))
        }
    }
}

pub fn print_welcome_msg(user_data: &UserData) {
    println!();
    println!("Welcome: {}!", user_data.get_name());
    println!("{}", user_data);
    println!();
}

pub fn start_game<I: InputSource>(input_source: &mut I, user_data: &mut UserData) {
    start_game_logic(input_source, user_data, rand::rng().random_range(1..=100));
}

pub fn start_game_logic<I: InputSource>(
    input_source: &mut I,
    user_data: &mut UserData,
    random_number: i8,
) {
    loop {
        println!("Please input your guess.");

        let guess_trimmed = match input_source.read_line() {
            Ok(input) => input,
            Err(game_error) => {
                // error continue, exit return
                if handle_input_error(game_error) {
                    continue;
                } else {
                    return;
                }
            }
        };

        let number = match guess_trimmed.parse::<i8>() {
            Ok(num) => num,
            Err(_) => {
                println!();
                println!("Please input a number!");
                println!();
                continue;
            }
        };

        match number.cmp(&random_number) {
            Ordering::Less => {
                println!();
                println!("Too small!");
                println!();
                user_data.record_losses();
            }
            Ordering::Equal => {
                println!("You win!");
                user_data.record_win();
                println!();
                println!("Now your game profile:");
                println!("{}", user_data);
                println!();
                println!("Thanks for playing!");
                println!();
                break;
            }
            Ordering::Greater => {
                println!();
                println!("Too big!");
                println!();
                user_data.record_losses();
            }
        }
    }
}
