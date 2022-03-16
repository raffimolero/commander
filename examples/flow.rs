use commander::*;

fn main() {
    let x = 5;
    prompt("Welcome to the program!");

    menu!("Welcome!" => {
        "test" => menu!("Testing." => {
            "cancel" => "ok"
            "back": "Goes back" => "Backing out..."
        })
        "" => "What?"
        "print": "prints stuff" => select!("Print what?" => {
            "yes" => "no"
            "no" => "yes"
            "xd" => queue(["no", "", "back"])
            "loop": "do not." => {
                println!("this was a mistake.");
                queue(["print", "loop"])
            }
            "quit": "quit program." => queue(["back", "back"])
        })
    });

    menu!("Hello there" => {
        "hi": format!("idk the num is {x}") => "Hello!"
        "hello": "makes response" => "Hi"
        "say": "says stuff" => select!("What do you want me to say?" => {
            "nothing" => "ok"
            "h" => "h"
        })
    });

    prompt("Goodbye!");
}
