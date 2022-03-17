fn main() {
    navigator::navigator!(ctx => {
        pick!("TEST" => {
            "run": "This should raise an automation error." => ctx.execute(["asdf"])
        });
        pick!("RUN" => {
            "break" => break
        });
    });
}
