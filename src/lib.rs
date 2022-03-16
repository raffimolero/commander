use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
};

struct Context {
    // path: Vec<String>,
    queue: Vec<String>,
}

static mut CONTEXT: Context = Context {
    // path: vec![],
    queue: vec![],
};

#[cfg(debug_assertions)]
pub fn _something_queued() -> bool {
    !unsafe { CONTEXT.queue.is_empty() }
}

#[cfg(debug_assertions)]
pub fn _queue_panic(command: &str) {
    panic!(
        "AUTOMATION ERROR!\nLast command: {command}\nqueue: {:?}\nNote: The queue is a stack. Read it in reverse.",
        unsafe { &CONTEXT.queue }
    );
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

// TODO: macro? maybe not.
pub fn queue<Iter, I, T>(inputs: Iter)
where
    Iter: IntoIterator<Item = T, IntoIter = I>,
    I: Iterator<Item = T> + DoubleEndedIterator,
    T: Into<String>,
{
    unsafe {
        CONTEXT
            .queue
            .extend(inputs.into_iter().map(|x| x.into()).rev());
    }
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
            #[cfg(debug_assertions)]
            let queued = commander::_something_queued();
            match unsafe { commander::_get_line($message, Some(&options)) }.trim() {
                $($option => {
                    commander::menu!(code $code);
                    if !$loop || $option == "back" || $option == "cancel"
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
                    #[cfg(debug_assertions)]
                    if queued {
                        commander::_queue_panic("");
                    }
                    commander::prompt("Please choose an option.");
                },
                unknown_cmd => {
                    #[cfg(debug_assertions)]
                    if queued {
                        commander::_queue_panic(unknown_cmd);
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
