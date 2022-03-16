use commander::*;

fn main() {
    let x = 5;

    let mut ctx = MenuContext::new();

    ctx.prompt("Welcome to the program!");

    menu!(ctx, "Welcome!" => {
        "test" => menu!(ctx, "Testing." => {
            "cancel" => ctx.prompt("ok")
            "back": "Goes back" => ctx.prompt("Backing out...")
        })
        "" => ctx.prompt("What?")
        "print": "prints stuff" => select!(ctx, "Print what?" => {
            "yes" => ctx.prompt("no")
            "no" => ctx.prompt("yes")
            "xd" => ctx.queue(["print", "no", "", "back"])
            "loop": "do not." => {
                println!("this was a mistake.");
                ctx.queue(["print", "loop"])
            }
            "quit": "quit program." => ctx.queue(["back", "back"])
        })
    });

    menu!(ctx, "Hello there" => {
        "hi": format!("idk the num is {x}") => ctx.prompt("Hello!")
        "hello": "makes response" => ctx.prompt("Hi")
        "say": "says stuff" => select!(ctx,"What do you want me to say?" => {
            "nothing" => ctx.prompt("ok")
            "h" => ctx.prompt("h")
            "a number" => {
                let num = ctx.get_line("Enter a number.").0.trim().parse::<i32>().expect("whoopsie, no error handling :P");
                ctx.prompt(num);
            }
        })
    });

    ctx.prompt("Goodbye!");
}
