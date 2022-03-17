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
impl Command {
    fn new(line: impl Display) -> Self {
        Self {
            command: line.to_string(),
            source: Source::Auto,
            prompt_level: PromptLevel::Hide,
        }
    }

    fn from_user(prompt: impl Display, cue: impl Display) -> Self {
        println!("{prompt}");
        let line = input_line(&cue);
        print_bar(DEFAULT_BAR_LENGTH);
        Command {
            command: line,
            source: Source::User,
            prompt_level: PromptLevel::Pause,
        }
    }

    fn from_auto(
        ctx: &mut NavContext,
        mut line: String,
        prompt: impl Display,
        cue: impl Display,
    ) -> Self {
        let mut cmd = Command::new(&line);

        // ending with a newline means it is an automatic command that must display inputs and prompts.
        if !line.contains('\n') {
            return cmd;
        }
        let suffix = line.pop();
        line = line.trim().to_string();
        cmd.command = line;
        match suffix {
            // default user input, overrideable.
            Some('?') => {
                cmd.prompt_level = PromptLevel::Pause;
                // prompt override.
                if !NavContext::new().confirm(
                    format!("{prompt}\n\nConfirm input: [{}]", cmd.command),
                    Some(true),
                ) {
                    // overriding will derail the rest of the script.
                    ctx.stack.clear();
                    cmd = Self::from_user(prompt, cue);
                }
            }
            // show prompts, pause.
            Some('.') => {
                cmd.prompt_level = PromptLevel::Pause;
                pause(format!("{prompt}\n=[AUTO]> {}", cmd.command));
            }
            // show prompts, no pauses.
            Some('\n') => {
                cmd.prompt_level = PromptLevel::Show;
                println!("{prompt}");
                print!("=[AUTO]> {}", cmd.command);
                print_bar(DEFAULT_BAR_LENGTH);
            }
            _ => ctx.error(&prompt),
        }

        cmd
    }
}

/// Currently just stores an input queue for the menu.
/// This allows certain menu options to input commands as if you typed them.
#[derive(Debug)]
pub struct NavContext {
    // path: Vec<String>,
    pub last_command: Command,
    pub stack: Vec<String>,
}
impl NavContext {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            last_command: Command {
                command: String::new(),
                source: Source::Auto,
                prompt_level: PromptLevel::Pause,
            },
        }
    }

    /// only reason it's public is cause macros.
    pub fn error(&self, prompt: impl Display) {
        panic!(
			"AUTOMATION ERROR!\nPrompt:\n{prompt}\n\nContext: {self:#?}\nNote: Read NavContext's stack in reverse.",
		);
    }

    pub fn confirm(&mut self, prompt: impl Display, default: Option<bool>) -> bool {
        let hint = default
            .map(|accept| format!(", Enter = {}", if accept { "y" } else { "y" }))
            .unwrap_or_default();
        let out = loop {
            let Command {
                command, source, ..
            } = self.next_command(&prompt, format!("=[y/n{hint}]> "));
            match command.to_ascii_lowercase().trim() {
                "y" | "yes" => break true,
                "n" | "no" => break false,
                "" => {
                    if let Some(out) = default {
                        break out;
                    }
                }
                _ => {}
            }
            if *source == Source::Auto {
                self.error(&prompt);
            }
        };
        out
    }

    pub fn next_command(&mut self, prompt: impl Display, cue: impl Display) -> &Command {
        self.last_command = self.stack.pop().map_or_else(
            || Command::from_user(&prompt, &cue),
            |line| Command::from_auto(self, line, &prompt, &cue),
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
        if self.last_command.prompt_level >= PromptLevel::Show {
            for line in message.to_string().lines() {
                println!("> {line}");
            }
            if self.last_command.prompt_level == PromptLevel::Pause {
                pause(DEFAULT_PAUSE_CUE);
            } else {
                print_bar(DEFAULT_BAR_LENGTH);
            }
        }
    }
}
