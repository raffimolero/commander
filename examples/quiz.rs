struct Question {
    question: &'static str,
    choices: [&'static str; 4],
    answer: &'static str,
}

fn main() {
    navigator::navigator!(ctx => {
        let test = [
            Question {
                question: "What is the first letter of the alphabet?",
                choices: [
                    "a",
                    "b",
                    "c",
                    "d"
                ],
                answer: "a",
            },
            Question {
                question: "What is your favorite programming language?",
                choices: [
                    "Malbolge",
                    "Rust",
                    "HTML",
                    "I don't code",
                ],
                answer: "b",
            },
            Question {
                question: "Hello there!",
                choices: [
                    "Hello!",
                    "Hi!",
                    "Hey!",
                    "General Kenobi.",
                ],
                answer: "d",
            },
            Question {
                question: "What sound does a duck make?",
                choices: [
                    "Quack",
                    "Beep",
                    "Ducks aren't real",
                    "We have your semicolon key hostage. Do as we say or face the consequences."
                ],
                answer: "a",
            },
            Question {
                question: "What happened on the 16th of March, 2022?",
                choices: [
                    "Nothing significant",
                    "The Earth rotated",
                    "The creator of this crate accidentally checked out a branch while in a detached head, and had to rewrite this entire quiz all over again",
                    "Ducks still aren't real"
                ],
                answer: "c",
            },
        ];

        let mut taken = false;
        nav!(if taken { "Welcome back!" } else { "What would you like to do?" } => {
            "quiz": format!("Take the quiz{}.", if taken { " again" } else { "" }) => {
                let mut canceled = false;
                pick!("How would you like to take the quiz?" => {
                    "manual" => ctx.prompt("Very well.")
                    "walkthrough": "Automatically answers the quiz for you." => {
                        // we're just going to steal the answers from each item...
                        // the "\n?" tells the program to let you see the input, and even override it if you want.
                        ctx.execute(test.iter().map(|question| question.answer.to_owned() + "\n?"))
                    }
                    // just "\n" will print the prompts, but you won't pause the program.
                    "a": "Answers 'a' for every question." => ctx.execute(["a\n", "a\n", "a\n", "a\n", "a\n"])
                    "back": "Cancels the quiz." => {
                        canceled = true;
                        ctx.prompt("Cancelled.");
                    }
                });

                if canceled {
                    continue;
                }
                taken = true;
                let mut score = 0;
                for Question { question, choices: [a, b, c, d], answer } in test.iter() {
                    let mut check_answer = |choice| {
                        if choice == *answer {
                            score += 1;
                            "Correct!"
                        } else {
                            "Wrong."
                        }
                    };
                    pick!(question => {
                        "a": a => ctx.prompt(check_answer("a"))
                        "b": b => ctx.prompt(check_answer("b"))
                        "c": c => ctx.prompt(check_answer("c"))
                        "d": d => ctx.prompt(check_answer("d"))
                    });
                }
                ctx.prompt(format!("Quiz finished!\nYou got {score} out of 5 questions right."));
            }
            "exit": "Exits the program." => break
        });
        ctx.prompt("Goodbye!");
    });
}
