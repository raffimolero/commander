use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineSource {
    User,
    Queue,
}

/// Currently just stores an input queue for the menu.
/// This allows certain menu options to input commands as if you typed them.
pub struct MenuContext {
    // path: Vec<String>,
    queue: Vec<String>,
}
impl MenuContext {
    pub fn new() -> Self {
        Self { queue: vec![] }
    }

    pub fn get_line(&mut self, prompt: impl Display) -> (String, LineSource) {
        // println!("QUEUE: {:?}", CONTEXT.queue);
        self.queue.pop().map_or_else(
            || {
                println!("> {prompt}");
                let line = input_line();
                print_bar();
                (line, LineSource::User)
            },
            |line| (line.to_string(), LineSource::Queue),
        )
    }

    pub fn get_queue(&self) -> &[String] {
        &self.queue
    }

    pub fn queue<Iter, I, T>(&mut self, inputs: Iter)
    where
        Iter: IntoIterator<Item = T, IntoIter = I>,
        I: Iterator<Item = T> + DoubleEndedIterator,
        T: Into<String>,
    {
        self.queue
            .extend(inputs.into_iter().map(|x| x.into()).rev());
    }

    pub fn prompt(&self, message: impl Display) {
        for line in message.to_string().split('\n') {
            println!("> {line}");
        }
        if self.queue.is_empty() {
            pause();
        }
    }
}

pub fn pause() {
    loop {
        println!("Press [Enter] to continue.");
        if input_line().trim().is_empty() {
            break;
        }
    }
    print_bar();
}

/// prints a bar in the console to separate stuff.
pub fn print_bar() {
    const BAR_LENGTH: usize = 32;
    println!("{}\n", "_".repeat(BAR_LENGTH));
}

pub fn input_line() -> String {
    let mut line = String::new();
    print!("=> ");
    stdout().flush().expect("Couldn't flush stdout.");
    stdin()
        .read_line(&mut line)
        .expect("Couldn't read line for some reason.");
    line
}

#[macro_export]
macro_rules! commander {
    ($context:ident => $tree:block) => {
        let mut $context = commander::MenuContext::new();
        macro_rules! dollar_workaround {
            ($S:tt) => {
                macro_rules! menu {
                    (panic $last_command:expr) => {
                        panic!(
                            "AUTOMATION ERROR!\nLast command: {}\nqueue: {:?}\nNote: The queue is a stack. Read it in reverse.",
                            $last_command,
                            $context.get_queue(),
                        );
                    };
                    {$message:expr => {
                        $S($option:literal $S(: $description:expr)? => $code:expr)+
                    }} => {
                        menu!(true, $message => {$S($option $S(: $description)? => $code)+})
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

                        loop {
                            let (line, source) = $context.get_line(&message);
                            match line.trim() {
                                $S($option => {
                                    let _: () = $code;
                                    if !$loop || $option == "back" || $option == "cancel"
                                    {
                                        break;
                                    }
                                })+
                                "back" => break,
                                "cancel" => {
                                    $context.prompt("Cancelled.");
                                    break;
                                }
                                // quit must be manually implemented in case there is data that needs to be managed
                                "" => {
                                    if options.is_empty() {
                                        break;
                                    }
                                    #[cfg(debug_assertions)]
                                    if source == commander::LineSource::Queue {
                                        menu!(panic "");
                                    }
                                    $context.prompt("Please choose an option.");
                                },
                                unknown_cmd => {
                                    #[cfg(debug_assertions)]
                                    if source == commander::LineSource::Queue {
                                        menu!(panic unknown_cmd);
                                    }
                                    $context.prompt("Unrecognized command.");
                                },
                            };
                        }
                    }};
                }

                macro_rules! pick {
                    {$message:expr => {$S($option:literal $S(: $description:expr)? => $code:expr)+}} => {
                        menu!(false, $message => {$S($option $S(: $description)? => $code)+})
                    };
                }
            }
        };
        dollar_workaround!($);
        $tree
    };
}
