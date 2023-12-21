use std::{
    io,
    io::IsTerminal,
    io::Write,
    process::{Command, Output},
};

#[derive(PartialEq, Debug)]
struct Cmd<'a> {
    binary: &'a str,
    args: Vec<&'a str>,
}

impl<'a> Cmd<'a> {
    fn from_statement(statement: &'a str) -> Option<Self> {
        let mut parts = statement.split_whitespace();
        parts.next().map(|binary| Cmd {
            binary,
            args: parts.collect(),
        })
    }

    fn run(self) -> Option<Output> {
        let child = Command::new(self.binary)
            .args(self.args)
            .spawn()
            .map_err(|e| eprintln!("{:?}", e))
            .ok()?;
        let output = child.wait_with_output().expect("command wasn't running");
        Some(output)
    }
}

fn cmds_from_line(line: &str) -> impl Iterator<Item = Cmd> {
    line.split(';').filter_map(Cmd::from_statement)
}

fn main() {
    loop {
        show_prompt();
        let line = read_line();
        for command in cmds_from_line(&line) {
            command.run();
        }
    }
}

/// If stdout is printed to a terminal, print a prompt.
/// Otherwise, do nothing. This allows to redirect the shell stdout
/// to a file or another process, without the prompt being printed.
fn show_prompt() {
    let mut stdout = std::io::stdout();
    if stdout.is_terminal() {
        write!(stdout, "> ").unwrap();
        // Flush stoud to ensure the prompt is displayed.
        stdout.flush().expect("can't flush stdout");
    }
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read line from stdin");
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_cmd_is_parsed_from_empty_line() {
        assert_eq!(Cmd::from_statement(""), None);
    }

    #[test]
    fn cmd_with_no_args_is_parsed() {
        assert_eq!(
            Cmd::from_statement("ls"),
            Some(Cmd {
                binary: "ls",
                args: vec![]
            })
        );
    }

    #[test]
    fn cmd_with_args_is_parsed() {
        assert_eq!(
            Cmd::from_statement("ls -l"),
            Some(Cmd {
                binary: "ls",
                args: vec!["-l"]
            })
        );
    }

    #[test]
    fn cmds_are_parsed() {
        let cmds: Vec<Cmd<'_>> = cmds_from_line("ls; echo hello").collect();
        assert_eq!(
            cmds,
            vec![
                Cmd {
                    binary: "ls",
                    args: vec![]
                },
                Cmd {
                    binary: "echo",
                    args: vec!["hello"]
                }
            ]
        );
    }
}
