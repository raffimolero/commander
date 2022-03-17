///! This is a place for me to mess around. Good luck understanding it :P

fn main() {
    let x = 5;

    navigator::navigator!(ctx => {
        pick!("pick one" => {
            "auto" => ctx.execute(["a\n?", "c\n.", "b\n", "d"])
            "no" => break
        });

        for _ in 0..4 {
            pick!("pick one" => {
                "a" => ctx.prompt("a")
                "b" => ctx.prompt("b")
                "c" => ctx.prompt("c")
                "d" => ctx.prompt("d")
            });
        }

        nav!("Welcome!" => {
            "test" => nav!("Testing." => {
                "cancel" => {
                    ctx.prompt("ok");
                    break;
                }
                "back": "Goes back" => {
                    ctx.prompt("Backing out...");
                    break;
                }
            })
            "" => ctx.prompt("What?")
            "print": "prints stuff" => pick!("Print what?" => {
                "yes" => ctx.prompt("no")
                "no" => ctx.prompt("yes")
                "xd" => ctx.execute(["print", "no", "", "break"])
                "loop": "do not." => {
                    ctx.prompt("you will never escape.");
                    ctx.execute(["print", "loop"]);
                }
                "quit": "quit program." => ctx.execute(["break", "break"])
            })
            "back" => break
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
            "back" => break
        });

        ctx.prompt("Goodbye!");
    });
}
