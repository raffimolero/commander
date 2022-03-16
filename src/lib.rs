use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
};

pub const DEFAULT_BAR_LENGTH: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    User,
    Stack,
}

pub struct Command {
    pub command: String,
    pub source: Source,
}

/// Currently just stores an input queue for the menu.
/// This allows certain menu options to input commands as if you typed them.
pub struct NavContext {
    // path: Vec<String>,
    pub stack: Vec<String>,
    pub last_command: Command,
}
impl NavContext {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            last_command: Command {
                command: String::new(),
                source: Source::Stack,
            },
        }
    }

    pub fn next_command(&mut self, prompt: impl Display) -> &Command {
        self.last_command = self.stack.pop().map_or_else(
            || {
                println!("> {prompt}");
                let line = input_line();
                print_bar(DEFAULT_BAR_LENGTH);
                Command {
                    command: line,
                    source: Source::User,
                }
            },
            |line| Command {
                command: line.to_string(),
                source: Source::Stack,
            },
        );
        &self.last_command
    }

    pub fn get_stack(&self) -> &[String] {
        &self.stack
    }

    pub fn execute<Iter, I, T>(&mut self, inputs: Iter)
    where
        Iter: IntoIterator<Item = T, IntoIter = I>,
        I: Iterator<Item = T> + DoubleEndedIterator,
        T: Into<String>,
    {
        self.stack
            .extend(inputs.into_iter().map(|x| x.into()).rev());
    }

    pub fn prompt(&self, message: impl Display) {
        if self.last_command.source == Source::User {
            for line in message.to_string().split('\n') {
                println!("> {line}");
            }
            pause();
        }
    }
}

/// Pauses the program to ask for the user's input.
pub fn pause() {
    loop {
        println!("Press [Enter] to continue.");
        if input_line().trim().is_empty() {
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
pub fn input_line() -> String {
    let mut line = String::new();
    print!("=> ");
    stdout().flush().expect("Couldn't flush stdout.");
    stdin()
        .read_line(&mut line)
        .expect("Couldn't read line for some reason.");
    line
}

/// The heart of the crate.
///
/// This macro will create a new menu context and *automagically* give you the other core macros which bind to this context:
///
/// `pick!`: Asks the user a question, and gives them a set of choices.
/// The user picks a choice, the program executes that choice, and moves on.
/// ```
/// pick!("message" => {
///     "option 1" => "action 1"
///     "option 2": "description" => "action 2"
/// });
/// ```
///
/// `nav!`: Works exactly like `pick!`, but instead of moving on, it loops and asks a possibly dynamically generated question forever.
/// To exit a nav, the user must type "back".
///
/// **Note: You can customize the "back" option to do whatever you want.** Just note that it will immediately queue a *back command* before any of yours.
#[macro_export]
macro_rules! navigator {
    ($context:ident => $tree:block) => {
        let mut $context = navigator::NavContext::new();
        macro_rules! dollar_workaround {
            ($S:tt) => {
                macro_rules! nav {
                    {$message:expr => {
                        $S($option:literal $S(: $description:expr)? => $code:expr)+
                    }} => {
                        nav!(true, $message => {$S($option $S(: $description)? => $code)+})
                    };
                    {$loop:literal, $message:expr => {
                        $S($option:literal $S(: $description:expr)? => $code:expr)+
                    }} => {{
                        let options = vec![
                            $S({
                                let mut desc = String::new();
                                $S(desc = format!(": {}", $description);)?
                                format!("[{}]{desc}", $option)
                            }),+
                        ];

                        fn error(context: &navigator::NavContext, options: &[String]) {
                            panic!(
                                "AUTOMATION ERROR!\nExpected options: {options:?}\nLast command: {}\nstack: {:?}\nNote: Read stack in reverse.",
								context.last_command.command,
                                context.stack,
                            );
                        }

                        let option_str = if options.is_empty() {
                            String::new()
                        } else {
                            format!("\n \n -- Options --\n{}", &options.join("\n"))
                        };

                        loop {
                            let mut message = $message.to_string() + &option_str;

                            let navigator::Command { command, source } = $context.next_command(&message);
                            match command.trim() {
                                $S($option => {
                                    let _: () = $code;
                                    if !$loop || $option == "back" {
                                        break;
                                    }
                                })+
                                "back" => break,
                                // quit must be manually implemented in case there is data that needs to be managed.
                                "" => {
                                    if options.is_empty() {
                                        break;
                                    }
                                    if *source == navigator::Source::Stack {
                                        error(&$context, &options);
                                    }
                                    $context.prompt("Please choose an option.");
                                },
                                unknown_cmd => {
                                    if *source == navigator::Source::Stack {
                                        error(&$context, &options);
                                    }
                                    $context.prompt("Unrecognized command.");
                                },
                            };
                        }
                    }};
                }

                macro_rules! pick {
                    {$message:expr => {$S($option:literal $S(: $description:expr)? => $code:expr)+}} => {
                        nav!(false, $message => {$S($option $S(: $description)? => $code)+})
                    };
                }
            }
        };
        dollar_workaround!($);
        $tree
    };
}
