use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
};

pub const DEFAULT_BAR_LENGTH: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineSource {
    User,
    Queue,
}

/// Currently just stores an input queue for the menu.
/// This allows certain menu options to input commands as if you typed them.
pub struct NavContext {
    // path: Vec<String>,
    pub stack: Vec<String>,
}
impl NavContext {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn get_line(&mut self, prompt: impl Display) -> (String, LineSource) {
        self.stack.pop().map_or_else(
            || {
                println!("> {prompt}");
                let line = input_line();
                print_bar(DEFAULT_BAR_LENGTH);
                (line, LineSource::User)
            },
            |line| (line.to_string(), LineSource::Queue),
        )
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
        for line in message.to_string().split('\n') {
            println!("> {line}");
        }
        if self.stack.is_empty() {
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
/// If you want to be able to programmatically queue inputs, use `NavContext.get_line()`.
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
/// `pick!`: Asks
/// ```
/// // Just like menu, but it doesn't loop.
/// pick!("message" => {
///     "option 1" => "action 1"
///     "option 2": "description" => "action 2"
/// });
/// ```
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

                        let mut message = $message.to_string();
                        if !options.is_empty() {
                            message.push_str("\n -- Options --\n");
                            message.push_str(&options.join("\n"));
                        };

                        fn error(context: &navigator::NavContext, options: &[String], last_command: &str) {
                            panic!(
                                "AUTOMATION ERROR!\nExpected options: {options:?}\nLast command: {last_command}\nstack: {:?}\nNote: Read stack in reverse.",
                                context.stack,
                            );
                        }

                        loop {
                            let (line, source) = $context.get_line(&message);
                            match line.trim() {
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
                                    if source == navigator::LineSource::Queue {
                                        error(&$context, &options, "");
                                    }
                                    $context.prompt("Please choose an option.");
                                },
                                unknown_cmd => {
                                    if source == navigator::LineSource::Queue {
                                        error(&$context, &options, unknown_cmd);
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
