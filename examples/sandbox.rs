fn main() {
    let x = 5;

    navigator::navigator!(ctx => {
        pick!("pick one" => {
            "auto" => ctx.execute(["a", "c", "b"])
        });

        pick!("pick one" => {
            "a" => ctx.prompt("a")
            "b" => ctx.prompt("b")
            "c" => ctx.prompt("c")
        });
        pick!("pick one" => {
            "a" => ctx.prompt("a")
            "b" => ctx.prompt("b")
            "c" => ctx.prompt("c")
        });
        pick!("pick one" => {
            "a" => ctx.prompt("a")
            "b" => ctx.prompt("b")
            "c" => ctx.prompt("c")
        });

        nav!("Welcome!" => {
            "test" => nav!("Testing." => {
                "cancel" => ctx.prompt("ok")
                "back": "Goes back" => ctx.prompt("Backing out...")
            })
            "" => ctx.prompt("What?")
            "print": "prints stuff" => pick!("Print what?" => {
                "yes" => ctx.prompt("no")
                "no" => ctx.prompt("yes")
                "xd" => ctx.execute(["print", "no", "", "back"])
                "loop": "do not." => {
                    ctx.prompt("you will never escape.");
                    ctx.execute(["print", "loop"]);
                }
                "quit": "quit program." => ctx.execute(["back", "back"])
            })
        });

        nav!("Hello there" => {
            "hi": format!("idk the num is {x}") => ctx.prompt("Hello!")
            "hello": "makes response" => ctx.prompt("Hi")
            "general kenobi": "reference" => ctx.prompt("i don't remember how the rest of the meme goes")
            "say": "says stuff" => pick!("What do you want me to say?" => {
                "nothing" => ctx.prompt("ok")
                "h" => ctx.prompt("h")
                "a number" => {
                    // let num = ctx.get_line("Enter a number.").0.trim().parse::<i32>();
                    // ctx.prompt(num);
                }
            })
        });

        ctx.prompt("Goodbye!");
    });
}
