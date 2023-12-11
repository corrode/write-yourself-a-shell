use std::{io, process::Command};

use anyhow::Context;

enum CommandResult {
    Success,
    Quit,
}

fn main_loop() -> anyhow::Result<CommandResult> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut parts = line.split_whitespace();
    let command = parts.next().context("No command given")?;
    if command == "quit" {
        return Ok(CommandResult::Quit);
    }
    let args: Vec<&str> = parts.collect();
    Command::new(command)
        .args(args)
        .spawn()
        .with_context(|| format!("Failed to execute {}", command))?;
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
