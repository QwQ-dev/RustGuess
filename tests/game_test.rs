use crate::input::mock_input_source::MockInputSource;
use rust_guess::game_error::GameError;
use rust_guess::game_handler::start_game_logic;
use rust_guess::input::InputSource;
use rust_guess::user_data::UserData;

mod input;

#[test]
fn game_test() {
    const SECRET_NUMBER: i8 = 42;

    let raw_inputs = vec![
        "10".to_string(),   // Too small (Loss)
        "80".to_string(),   // Too big (Loss)
        "abc".to_string(),  // Invalid Number (Continue)
        "42".to_string(),   // Correct (Win)
        "quit".to_string(), // Should not be read
    ];

    let mut mock_source = MockInputSource {
        inputs: raw_inputs.into_iter(),
    };
    let mut user = UserData::with_name(String::from("TestRunner"));

    start_game_logic(&mut mock_source, &mut user, SECRET_NUMBER);

    assert_eq!(user.wins, 1, "User should have exactly 1 win.");
    assert_eq!(user.losses, 2, "User should have exactly 2 losses.");

    let next_input = mock_source.read_line();

    assert!(
        next_input.is_err(),
        "Input should be exhausted or returned ExitCommand."
    );
    assert_eq!(
        next_input.unwrap_err(),
        GameError::ExitCommand,
        "Test should end with ExitCommand."
    );

    // 1win 2los
    assert_eq!(user.wins, 1, "User should have exactly 1 wins.");
    assert_eq!(user.losses, 2, "User should have exactly 2 losses.");
}
