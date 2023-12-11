use std::{
    io::Write, // <--- bring flush() into scope
    io::{self},
    process::Command,
};

enum CommandResult {
    Success,
    Quit,
}

fn main_loop() -> Result<CommandResult, ()> {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut parts = line.split_whitespace();

    // If no command was entered, execute nothing.
    if let Some(command) = parts.next() {
        if command == "quit" {
            return Ok(CommandResult::Quit);
        }
        let args: Vec<&str> = parts.collect();
        if let Err(err) = Command::new(command).args(args).spawn() {
            eprintln!("{:?}", err);
        }
    }
    Ok(CommandResult::Success)
}

fn main() {
    loop {
        match main_loop() {
            Ok(CommandResult::Quit) => break,
            Ok(CommandResult::Success) => {}
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
}
