use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
};

struct Context {
    // path: Vec<String>,
    queue: Vec<&'static str>,
}

static mut CONTEXT: Context = Context {
    // path: vec![],
    queue: vec![],
};

#[cfg(debug_assertions)]
pub fn _queue_error_check(command: &str) {
    if !unsafe { CONTEXT.queue.is_empty() } {
        panic!(
            "AUTOMATION ERROR!\nLast command: {command}\nqueue: {:?}",
            unsafe { &CONTEXT.queue }
        );
    }
}

pub fn get_line() -> String {
    let mut line = String::new();
    stdout().flush().expect("Couldn't flush stdout.");
    stdin()
        .read_line(&mut line)
        .expect("Couldn't read line for some reason.");
    line
}

pub unsafe fn _get_line(prompt: impl Display, options: Option<&[String]>) -> String {
    // println!("QUEUE: {:?}", CONTEXT.queue);
    CONTEXT.queue.pop().map_or_else(
        || {
            println!("> {prompt}");
            if let Some(options) = options {
                if !options.is_empty() {
                    println!("  -- Options --");
                    for option in options {
                        println!("{option}");
                    }
                }
            }
            let line = get_line();
            println!();
            line
        },
        |line| line.to_string(),
    )
}

pub trait Command {
    /// internally used by the macros
    unsafe fn _queue(self) -> bool;
}

impl Command for () {
    unsafe fn _queue(self) -> bool {
        false
    }
}
impl<const N: usize> Command for [&'static str; N] {
    unsafe fn _queue(self) -> bool {
        CONTEXT.queue.extend(self.into_iter().rev());
        true
    }
}

#[macro_export]
macro_rules! queue {
    () => {};
}

#[macro_export]
macro_rules! input {
    ($message:expr => $type:ty) => {
        println!("{}", message);
        unsafe { commander::_get_line().trim().parse::<$type>() }
    };
    ($type:ty) => {
        unsafe { commander::_get_line().trim().parse::<$type>() }
    };
    () => {
        unsafe { commander::_get_line().trim().parse() }
    };
}

pub fn prompt(message: impl Display) {
    println!("> {message}");
    if unsafe { CONTEXT.queue.is_empty() } {
        loop {
            println!("Press [Enter] to continue.");
            if get_line().trim().is_empty() {
                break;
            }
        }
    }
}

#[macro_export]
macro_rules! menu {
    (code $content:literal) => {
        commander::prompt($content);
    };
    (code $code:expr) => {
        $code
    };
    {$message:expr => {
        $($option:literal $(: $description:expr)? => $code:expr)+
    }} => {
        commander::menu!(true, $message => {$($option $(: $description)? => $code)+})
    };
    {$loop:literal, $message:expr => {
        $($option:literal $(: $description:expr)? => $code:expr)+
    }} => {{
        let options = vec![
            $({
                let mut desc = String::new();
                $(desc = format!(": {}", $description);)?
                format!("[{}]{desc}", $option)
            }),+
        ];
        loop {
            match unsafe { commander::_get_line($message, Some(&options)) }.trim() {
                $($option => {
                    if !unsafe { commander::Command::_queue(commander::menu!(code $code)) }
                        && (!$loop || $option == "back" || $option == "cancel")
                    {
                        break;
                    }
                })+
                "back" => break,
                "cancel" => {
                    commander::prompt("Cancelled.");
                    break;
                }
                // quit must be manually implemented in case there is data that needs to be managed
                "" => {
                    if options.is_empty() {
                        break;
                    }
                    commander::_queue_error_check("");
                    commander::prompt("Please choose an option.");
                },
                unknown_cmd => {
                    #[cfg(debug_assertions)] {
                        commander::_queue_error_check(unknown_cmd);
                    }
                    commander::prompt("Unrecognized command.");
                },
            };
        }
    }};
}

#[macro_export]
macro_rules! select {
    {$message:expr => {$($option:literal $(: $description:expr)? => $code:expr)+}} => {
        commander::menu!(false, $message => {$($option $(: $description)? => $code)+})
    };
}
