use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
};

pub const DEFAULT_OPTION_HEADER: &'static str = "   -- [ Options ] --";
pub const DEFAULT_BAR_LENGTH: usize = 32;
pub const DEFAULT_PAUSE_MESSAGE: &'static str = "Press [Enter] to continue.";
pub const DEFAULT_USER_INPUT_CUE: &'static str = "=> ";

/// Pauses the program to ask for the user's input.
pub fn pause(message: impl Display, cue: impl Display) {
    loop {
        println!("{message}");
        if input_line(&cue).trim().is_empty() {
            break;
        }
    }
    print_bar(DEFAULT_BAR_LENGTH);
}

/// Prints a separator bar in the console.
pub fn print_bar(length: usize) {
    println!("{}\n", "_".repeat(length));
}

/// Takes input directly from user.
///
/// If you want to be able to programmatically queue inputs, use `NavContext.next_command()`.
pub fn input_line(cue: impl Display) -> String {
    print!("{cue}");
    stdout().flush().expect("Couldn't flush stdout.");
    let mut line = String::new();
    stdin()
        .read_line(&mut line)
        .expect("Couldn't read line for some reason.");
    line
}
