use std::{
    io,
    io::IsTerminal,
    io::Write,
    process::{Command, Output},
};

// This struct doesn't use lifetimes to keep the code simple.
// You can try to use `&str` instead of `String` to avoid unnecessary allocations. 👍
#[derive(PartialEq, Debug)]
struct Cmd {
    binary: String,
    args: Vec<String>,
}

#[derive(PartialEq, Debug)]
enum Element {
    /// `&&`
    And,
    /// `||`
    Or,
    /// Command.
    Cmd(Cmd),
}

/// Parse `[Element]`s from a string.
struct Parser {
    current: usize,
    tokens: Vec<String>,
}

impl Parser {
    fn new(chain: &str) -> Self {
        Self {
            tokens: chain.split_whitespace().map(String::from).collect(),
            current: 0,
        }
    }

    fn parse(mut self) -> Option<Chain> {
        let mut elements = vec![];
        while let Some(e) = self.parse_next() {
            elements.push(e);
        }
        if !elements.is_empty() {
            Some(Chain { elements })
        } else {
            None
        }
    }

    fn parse_next(&mut self) -> Option<Element> {
        let next = self.tokens.get(self.current).map(|s| s.to_string());
        next.and_then(|next| {
            self.current += 1;
            match Element::parse_operator(&next) {
                Some(operator) => Some(operator),
                None => self.parse_cmd(next.to_string()).map(Element::Cmd),
            }
        })
    }

    fn parse_cmd(&mut self, binary: String) -> Option<Cmd> {
        let mut args: Vec<String> = vec![];
        loop {
            let next = self.tokens.get(self.current);
            match next {
                Some(token) if Element::is_operator(token) => {
                    // found operator, so I already parsed all cmd
                    break;
                }
                Some(token) => {
                    args.push(token.to_string());
                }
                None => break,
            }
            self.current += 1;
        }
        Some(Cmd { binary, args })
    }
}

#[derive(PartialEq, Debug)]
struct Chain {
    elements: Vec<Element>,
}

impl Chain {
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

impl Element {
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
}

impl Cmd {
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
        let chains = chains_from_line(line);
        for s in chains {
            s.run();
        }
    }
}

/// If `stdout` is printed to a terminal, print a prompt.
/// Otherwise, do nothing. This allows to redirect the shell `stdout`
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

fn chains_from_line(line: String) -> Vec<Chain> {
    // For simplicity's sake, this workshop uses the split function.
    // This is inefficient because it parses the whole line.
    // If you feel adventurous, try to parse the line character by character instead. 🤠
    line.split(';')
        .map(|s| s.to_string())
        .filter_map(|s| Parser::new(&s).parse())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_chains(line: &str) -> Vec<Chain> {
        chains_from_line(line.to_string())
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
                    binary: "ls".to_string(),
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
                    binary: "ls".to_string(),
                    args: vec!["-l".to_string()]
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
                        binary: "ls".to_string(),
                        args: vec![]
                    }),]
                },
                Chain {
                    elements: vec![Element::Cmd(Cmd {
                        binary: "echo".to_string(),
                        args: vec!["hello".to_string()]
                    }),]
                },
            ]
        );
    }
}
