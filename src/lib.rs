use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
};

pub const DEFAULT_BAR_LENGTH: usize = 32;
pub const DEFAULT_PAUSE_MESSAGE: &'static str = "Press [Enter] to continue.";
pub const DEFAULT_USER_INPUT_CUE: &'static str = "=> ";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    User,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PromptLevel {
    Hide = 0,
    Show = 1,
    Pause = 2,
}

#[derive(Debug)]
pub struct Command {
    pub command: String,
    pub source: Source,
    pub prompt_level: PromptLevel,
}

/// Currently just stores an input queue for the menu.
/// This allows certain menu options to input commands as if you typed them.
#[derive(Debug)]
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
                source: Source::Auto,
                prompt_level: PromptLevel::Hide,
            },
        }
    }

    /// only reason it's public is cause macros.
    pub fn error(&self, options: impl Display) {
        panic!(
			"AUTOMATION ERROR!\nExpected options: {options}\nLast command: {}\nstack: {:?}\nNote: Read stack in reverse.",
			self.last_command.command,
			self.stack,
		);
    }

    pub fn next_command(&mut self, prompt: impl Display) -> &Command {
        let get_input = || {
            println!("{prompt}");
            let line = input_line(DEFAULT_USER_INPUT_CUE);
            print_bar(DEFAULT_BAR_LENGTH);
            Command {
                command: line,
                source: Source::User,
                prompt_level: PromptLevel::Pause,
            }
        };
        self.last_command = self.stack.pop().map_or_else(get_input, |mut line| {
            let mut cmd = Command {
                command: line.to_string(),
                source: Source::Auto,
                prompt_level: PromptLevel::Hide,
            };

            // ending with a newline means it is an automatic command that must display inputs and prompts.
            if line.contains('\n') {
                println!("{prompt}");
                match line.pop() {
                    // default user input, overrideable.
                    Some('?') => {
                        cmd = Command {
                            command: line.trim().to_string(),
                            source: Source::User,
                            prompt_level: PromptLevel::Pause,
                        };
                        // prompt override.
                        use crate as navigator;
                        navigator!(ctx => {
                            println!();
                            pick!(format!("Scripted user input: {:?}\nAccept?", cmd.command) => {
                                "": format!("Accept input: [{}]", cmd.command) => {}
                                "yes" => {}
                                "cancel": "Make your own inputs." => {
                                    self.stack.clear();
                                    cmd = get_input();
                                }
                            });
                        });
                    }
                    // show prompts, pause.
                    Some('.') => {
                        // remove that newline.
                        cmd.command.pop();
                        cmd.prompt_level = PromptLevel::Pause;
                        print_bar(DEFAULT_BAR_LENGTH);
                    }
                    // show prompts, no pauses.
                    Some('\n') => {
                        cmd.prompt_level = PromptLevel::Show;
                        println!("=[AUTO]> {line}");
                        print_bar(DEFAULT_BAR_LENGTH);
                    }
                    _ => self.error(&prompt),
                }
            }

            cmd
        });
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
        if self.last_command.prompt_level >= PromptLevel::Show {
            for line in message.to_string().lines() {
                println!("> {line}");
            }
            if self.last_command.prompt_level == PromptLevel::Pause {
                pause(DEFAULT_PAUSE_MESSAGE, DEFAULT_USER_INPUT_CUE);
            } else {
                print_bar(DEFAULT_BAR_LENGTH);
            }
        }
    }
}

/// Pauses the program to ask for the user's input.
pub fn pause(message: &str, cue: &str) {
    loop {
        println!("{message}");
        if input_line(cue).trim().is_empty() {
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
pub fn input_line(cue: &str) -> String {
    print!("{cue}");
    stdout().flush().expect("Couldn't flush stdout.");
    let mut line = String::new();
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
/// To exit a nav, the user must type "break".
///
/// **Note: You can customize the "break" option to do whatever you want.** Just note that it will immediately queue a *break command* before any of yours.
#[macro_export]
macro_rules! navigator {
    ($context:ident => $tree:block) => {
        let mut $context = navigator::NavContext::new();
        macro_rules! dollar_workaround { ($S:tt) => {
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
							#[allow(unused_assignments)]
							#[allow(unused_mut)]
							let mut desc = String::new();
							$S(desc = format!(": {}", $description);)?
							format!("[{}]{desc}", $option)
						}),+
					];

					let option_str = if options.is_empty() {
						String::new()
					} else {
						format!("\n -- [ Options ] --\n{}", &options.join("\n"))
					};

					loop {
						let message = $message
							.to_string()
							.lines()
							.map(|s| format!("> {s}"))
							.collect::<Vec<_>>()
							.join("\n") + &option_str;

						let navigator::Command { command, source, .. } = $context.next_command(&message);
						#[allow(unreachable_patterns)]
						match command.trim() {
							$S($option => {
								let _: () = $code;
								if !$loop || $option == "break" {
									break;
								}
							})+
							"break" => break,
							"" => {
								if options.is_empty() {
									break;
								}
								if *source == navigator::Source::Auto {
									$context.error(&message);
								}
								$context.prompt("Please choose an option.");
							},
							_ => {
								if *source == navigator::Source::Auto {
									$context.error(&message);
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
		}} dollar_workaround!($);
        $tree
    };
}
