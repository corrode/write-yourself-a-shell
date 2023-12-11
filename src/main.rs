use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use std::convert::TryFrom;
use std::process::{Command, Stdio};

mod builtin;
mod cmd;
mod error;

use cmd::{Statement, Statements};
use error::Error;

fn main() -> Result<(), Error> {
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                match handle(line) {
                    Err(Error::NoBinary) | Ok(()) => {}
                    Err(e) => eprintln!("rush: {:?}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {}
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

fn handle(line: String) -> Result<(), Error> {
    let statements = Statements::try_from(line.as_ref())?;
    for statement in statements {
        match statement {
            Statement::Cmd { binary, args } => {
                if binary == "exit" {
                    builtin::exit(args);
                } else {
                    match Command::new(binary).args(args).output() {
                        Ok(output) => print!("{}", String::from_utf8_lossy(&output.stdout)),
                        Err(e) => eprintln!("rush: {:?}", e),
                    }
                }
            }

            Statement::Pipe { input, output } => match (*input, *output) {
                (
                    Statement::Cmd {
                        binary: binary_in,
                        args: args_in,
                    },
                    Statement::Cmd {
                        binary: binary_out,
                        args: args_out,
                    },
                ) => {
                    let first_child = Command::new(binary_in)
                        .args(args_in)
                        .stdout(Stdio::piped())
                        .spawn()?;
                    let first_out = first_child.stdout.ok_or_else(|| Error::Spawn)?;
                    let second_child = Command::new(binary_out)
                        .args(args_out)
                        .stdin(Stdio::from(first_out))
                        .stdout(Stdio::piped())
                        .spawn()?;
                    let output = second_child.wait_with_output()?;
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
                _ => unimplemented!(),
            },
        }
    }
    Ok(())
}
