use std::fmt::Display;

use crate::helpers::*;

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
                        crate::navigator!(ctx => {
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
