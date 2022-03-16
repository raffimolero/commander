/// The heart of the crate.
///
/// This macro will create a new menu context and *automagically* give you the other core macros which bind to this context:
///
/// `pick!`: Asks the user a question, and gives them a set of choices.
/// The user picks a choice, the program executes that choice, and moves on.
/// ```
/// pick!("message" => {
///     "option 1" => "action 1"
///     "option 2": "description" => "action 2"
/// });
/// ```
///
/// `nav!`: Works exactly like `pick!`, but instead of moving on, it loops and asks a possibly dynamically generated question forever.
/// To exit a nav, the user must type "break".
///
/// **Note: You can customize the "break" option to do whatever you want.** Just note that it will immediately queue a *break command* before any of yours.
#[macro_export]
macro_rules! navigator {
    ($context:ident => $tree:block) => {
        let mut $context = navigator::context::NavContext::new();
        macro_rules! dollar_workaround { ($S:tt) => {
			macro_rules! nav {
				{$message:expr => {
					$S($option:literal $S(: $description:expr)? => $code:expr)+
				}} => {
					nav!(true, $message => {$S($option $S(: $description)? => $code)+})
				};
				{$loop:literal, $message:expr => {
					$S($option:literal $S(: $description:expr)? => $code:expr)+
				}} => {{
					let options = vec![
						$S({
							#[allow(unused_assignments)]
							#[allow(unused_mut)]
							let mut desc = String::new();
							$S(desc = format!(": {}", $description);)?
							format!("[{}]{desc}", $option)
						}),+
					];

					let option_str = if options.is_empty() {
						String::new()
					} else {
						format!("\n -- [ Options ] --\n{}", &options.join("\n"))
					};

					loop {
						let message = $message
							.to_string()
							.lines()
							.map(|s| format!("> {s}"))
							.collect::<Vec<_>>()
							.join("\n") + &option_str;

						let navigator::context::Command { command, source, .. } = $context.next_command(&message);
						#[allow(unreachable_patterns)]
						match command.trim() {
							$S($option => {
								let _: () = $code;
								if !$loop || $option == "break" {
									break;
								}
							})+
							"break" => break,
							"" => {
								if options.is_empty() {
									break;
								}
								if *source == navigator::context::Source::Auto {
									$context.error(&message);
								}
								$context.prompt("Please choose an option.");
							},
							_ => {
								if *source == navigator::context::Source::Auto {
									$context.error(&message);
								}
								$context.prompt("Unrecognized command.");
							},
						};
					}
				}};
			}

			macro_rules! pick {
				{$message:expr => {$S($option:literal $S(: $description:expr)? => $code:expr)+}} => {
					nav!(false, $message => {$S($option $S(: $description)? => $code)+})
				};
			}
		}} dollar_workaround!($);
        $tree
    };
}
