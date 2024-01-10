use std::{
    io,
    io::IsTerminal,
    io::Write,
    iter::Peekable,
    process::{Command, Output},
};

#[derive(PartialEq, Debug)]
struct Cmd<'a> {
    binary: &'a str,
    args: Vec<&'a str>,
}

#[derive(PartialEq, Debug)]
enum Element<'a> {
    /// `&&`
    And,
    /// `||`
    Or,
    /// Command.
    Cmd(Cmd<'a>),
}

#[derive(PartialEq, Debug)]
struct Chain<'a> {
    elements: Vec<Element<'a>>,
}

impl<'a> Chain<'a> {
    fn parse(command: &'a str) -> Option<Self> {
        let mut tokens = command.split_whitespace().peekable();
        let mut elements = vec![];
        while let Some(e) = Element::parse_next(&mut tokens) {
            elements.push(e);
        }
        if !elements.is_empty() {
            Some(Self { elements })
        } else {
            None
        }
    }

    fn run(self) {
        let mut prev_output: Option<Output> = None;
        for e in self.elements {
            match e {
                Element::Cmd(cmd) => {
                    prev_output = cmd.run();
                }
                Element::And => {
                    let status = prev_output.expect("no command before &&").status;
                    if !status.success() {
                        break;
                    }
                    prev_output = None;
                }
                Element::Or => {
                    let status = prev_output.expect("no command before ||").status;
                    if status.success() {
                        break;
                    }
                    prev_output = None;
                }
            }
        }
    }
}

impl<'a> Element<'a> {
    fn parse_next<I: Iterator<Item = &'a str>>(tokens: &mut Peekable<I>) -> Option<Self> {
        tokens
            .next()
            .and_then(|next| match Self::parse_operator(next) {
                Some(operator) => Some(operator),
                None => Self::parse_cmd(next, tokens).map(Self::Cmd),
            })
    }

    fn parse_operator(token: &str) -> Option<Self> {
        match token {
            "&&" => Some(Self::And),
            "||" => Some(Self::Or),
            _ => None,
        }
    }

    fn is_operator(token: &str) -> bool {
        Self::parse_operator(token).is_some()
    }

    fn parse_cmd<I: Iterator<Item = &'a str>>(
        binary: &'a str,
        tokens: &mut Peekable<I>,
    ) -> Option<Cmd<'a>> {
        let mut args: Vec<&str> = vec![];
        loop {
            let next = tokens.peek();
            match next {
                Some(token) if Self::is_operator(token) => {
                    // found operator, so I already parsed all cmd
                    break;
                }
                Some(token) => {
                    args.push(token);
                }
                None => break,
            }
            tokens.next();
        }
        Some(Cmd { binary, args })
    }
}

impl<'a> Cmd<'a> {
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

fn main() {
    loop {
        show_prompt();
        let line = read_line();
        let chains = chains_from_line(&line);
        for s in chains {
            s.run();
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

fn chains_from_line(line: &str) -> impl Iterator<Item = Chain> {
    let commands = line.split(';');
    commands.filter_map(Chain::parse)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_chains(line: &str) -> Vec<Chain> {
        chains_from_line(line).collect()
    }

    #[test]
    fn no_cmd_is_parsed_from_empty_line() {
        assert_eq!(parse_chains(""), vec![]);
    }

    #[test]
    fn cmd_with_no_args_is_parsed() {
        assert_eq!(
            parse_chains("ls"),
            vec![Chain {
                elements: vec![Element::Cmd(Cmd {
                    binary: "ls",
                    args: vec![]
                }),]
            },]
        );
    }

    #[test]
    fn cmd_with_args_is_parsed() {
        assert_eq!(
            parse_chains("ls -l"),
            vec![Chain {
                elements: vec![Element::Cmd(Cmd {
                    binary: "ls",
                    args: vec!["-l"]
                })]
            }]
        );
    }

    #[test]
    fn cmds_are_parsed() {
        assert_eq!(
            parse_chains("ls; echo hello"),
            vec![
                Chain {
                    elements: vec![Element::Cmd(Cmd {
                        binary: "ls",
                        args: vec![]
                    }),]
                },
                Chain {
                    elements: vec![Element::Cmd(Cmd {
                        binary: "echo",
                        args: vec!["hello"]
                    }),]
                },
            ]
        );
    }
}
