use commander::*;

fn main() {
    let x = 5;

    let mut ctx = MenuContext::new();

    ctx.prompt("Welcome to the program!");

    menu!(ctx, "Welcome!" => {
        "test" => menu!(ctx, "Testing." => {
            "cancel" => "ok"
            "back": "Goes back" => "Backing out..."
        })
        "" => "What?"
        "print": "prints stuff" => select!(ctx, "Print what?" => {
            "yes" => "no"
            "no" => "yes"
            "xd" => ctx.queue(["print", "no", "", "back"])
            "loop": "do not." => {
                println!("this was a mistake.");
                ctx.queue(["print", "loop"])
            }
            "quit": "quit program." => ctx.queue(["back", "back"])
        })
    });

    menu!(ctx, "Hello there" => {
        "hi": format!("idk the num is {x}") => "Hello!"
        "hello": "makes response" => "Hi"
        "say": "says stuff" => select!(ctx,"What do you want me to say?" => {
            "nothing" => "ok"
            "h" => "h"
            // "a number" => {
            //     num
            // }
        })
    });

    ctx.prompt("Goodbye!");
}
