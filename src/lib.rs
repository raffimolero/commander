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
        println!("> {message}");
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
macro_rules! menu {
    (panic $context:ident, $last_command:expr) => {
        panic!(
            "AUTOMATION ERROR!\nLast command: {}\nqueue: {:?}\nNote: The queue is a stack. Read it in reverse.",
            $last_command,
            $context.get_queue(),
        );
    };
    {$context:ident, $message:expr => {
        $($option:literal $(: $description:expr)? => $code:expr)+
    }} => {
        commander::menu!($context, true, $message => {$($option $(: $description)? => $code)+})
    };
    {$context:ident, $loop:literal, $message:expr => {
        $($option:literal $(: $description:expr)? => $code:expr)+
    }} => {{
        let options = vec![
            $({
                let mut desc = String::new();
                $(desc = format!(": {}", $description);)?
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
                $($option => {
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
                        menu!(panic $context, "");
                    }
                    $context.prompt("Please choose an option.");
                },
                unknown_cmd => {
                    #[cfg(debug_assertions)]
                    if source == commander::LineSource::Queue {
                        menu!(panic $context, unknown_cmd);
                    }
                    $context.prompt("Unrecognized command.");
                },
            };
        }
    }};
}

#[macro_export]
macro_rules! select {
    {$context:ident, $message:expr => {$($option:literal $(: $description:expr)? => $code:expr)+}} => {
        commander::menu!($context, false, $message => {$($option $(: $description)? => $code)+})
    };
}
