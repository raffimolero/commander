fn main() {
    commander::commander!(ctx => {
        let questions = [
            (
                "What is the first letter of the alphabet?",
                ["a", "b", "c", "d"],
            ),
            (
                "What is your favorite programming language?",
                ["Malbolge", "Rust", "HTML", "I don't code"],
            ),
            (
                "Hello there!",
                ["Hello!", "Hi!", "Hey!", "General Kenobi."],
            ),
            (
                "What sound does a duck make?",
                ["Quack", "Beep", "Ducks aren't real", "We have your semicolon key hostage. Do as we say or face the consequences."],
            ),
            (
                "What happened on the 16th of March, 2022?",
                ["Nothing significant", "The Earth rotated", "The creator of this crate accidentally checked out a branch while in a detached head, and had to write this entire quiz again", "Ducks aren't real"],
            ),
        ];
        let answers = ["a", "b", "d", "a", "c"];

        pick!("Would you like to take the quiz?" => {
            "yes" => ctx.prompt("Very well.")
            "no": "Automatically answers the quiz for you." => ctx.queue(answers)
            "a": "Answers 'a' for every question." => ctx.queue(["a", "a", "a", "a", "a"])
        });

        let mut score = 0;
        for ((question, [a, b, c, d]), answer) in questions.iter().zip(answers) {
            let mut check_answer = |choice| {
                if choice == answer {
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
    });
}
