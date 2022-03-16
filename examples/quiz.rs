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
                choices: ["a", "b", "c", "d"],
                answer: "a",
            },
            Question {
                question: "What is your favorite programming language?",
                choices: ["Malbolge", "Rust", "HTML", "I don't code"],
                answer: "b",
            },
            Question {
                question: "Hello there!",
                choices: ["Hello!", "Hi!", "Hey!", "General Kenobi."],
                answer: "d",
            },
            Question {
                question: "What sound does a duck make?",
                choices: ["Quack", "Beep", "Ducks aren't real", "We have your semicolon key hostage. Do as we say or face the consequences."],
                answer: "a",
            },
            Question {
                question: "What happened on the 16th of March, 2022?",
                choices: ["Nothing significant", "The Earth rotated", "The creator of this crate accidentally checked out a branch while in a detached head, and had to write this entire quiz again", "Ducks aren't real"],
                answer: "c",
            },
        ];

        let mut taken = false;
        nav!(format!("Would you like to take the quiz{}?", if taken { " again" } else { "" }) => {
            "yes" => {
                taken = true;
                pick!("How would you like to take the quiz?" => {
                    "manual" => ctx.prompt("Very well.")
                    "auto": "Automatically answers the quiz for you." => {
                        // we're just going to steal the answers from each item...
                        ctx.execute(test.iter().map(|question| question.answer))
                    }
                    "a": "Answers 'a' for every question." => ctx.execute(["a", "a", "a", "a", "a"])
                });
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
            "no": "Exits the program." => ctx.execute(["back"])
        });
        ctx.prompt("Goodbye!");
    });
}
