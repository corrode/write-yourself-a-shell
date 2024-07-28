use std::{
    io,
    io::IsTerminal,
    io::Write,
    process::{Command, Output},
};

/// Alias for our `Result` type. You could also use `anyhow` instead.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// This module contains the built-in commands of the shell.
/// In a production-grade project, you would probably want to
/// move this module to its own file, but we keep it here to have
/// everything in one file for learning purposes.
mod builtins {
    use crate::Result;
    use std::io::Write;
    use std::{path::PathBuf, process::Output};

    /// The `cd` command changes the current directory.
    ///
    /// The `cd` command changes the current directory of the shell.
    /// If the directory is not found, it prints an error message.
    /// If the directory is successfully changed, it returns `Ok(())` and
    /// the shell should update its current directory.
    ///
    /// A real `cd` accepts options like `-L` and `-P`, to resolve symbolic links.
    /// It also has special cases like `cd -` to go to the previous directory or `cd ~` to go to the home directory.
    /// We don't implement these features in this workshop, but you can give it a try!
    pub struct Cd {
        /// The directory to change into.
        dir: PathBuf,
    }

    impl Cd {
        /// Create a new `Cd` command.
        pub fn new(dir: PathBuf) -> Self {
            Self { dir }
        }

        /// Run the `cd` command.
        pub fn run(self) -> Result<Option<Output>> {
            // `std::env::set_current_dir` changes the current directory of the process
            // (our shell in this case).
            std::env::set_current_dir(&self.dir)?;
            // The `cd` command doesn't produce any output.
            Ok(None)
        }
    }

    /// The `exit` command exits the shell.
    ///
    /// The `exit` command exits the shell with the given status code.
    /// If no status code is given, it exits with status code 0.
    pub struct Exit {
        /// The status code to exit with.
        status: i32,
    }

    impl Exit {
        /// Create a new `Exit` command.
        pub fn new(status: i32) -> Self {
            Self { status }
        }

        /// Run the `exit` command.
        pub fn run(self) -> Result<Option<Output>> {
            // The `exit` command doesn't produce any output.
            std::process::exit(self.status);
        }
    }

    #[cfg(unix)]
    use std::os::unix::process::ExitStatusExt;

    #[cfg(windows)]
    use std::os::windows::process::ExitStatusExt;

    // Store history file in current path. This is convenient for debugging purposes.
    // In a real shell, the history would be stored in a file in the user's home directory.
    const DEFAULT_HISTORY_PATH: &str = ".history";

    /// The `history` command displays the command history.
    pub struct History {
        history_path: PathBuf,
    }

    impl History {
        /// Create a new `History` command.
        pub fn new() -> Self {
            // The path can be overridden by setting the `HISTORY_PATH` environment variable.
            let history_path = std::env::var("HISTORY_PATH")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from(DEFAULT_HISTORY_PATH));

            Self { history_path }
        }

        /// Add a command to the history.
        pub fn add(&self, command: &str) -> Result<()> {
            let mut history = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.history_path)?;
            writeln!(history, "{command}")?;
            Ok(())
        }

        /// Get all the commands in the history.
        pub fn run(self) -> Result<Option<Output>> {
            let history = std::fs::read_to_string(&self.history_path)?;

            Ok(Some(Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: history.into_bytes(),
                stderr: Vec::new(),
            }))
        }
    }
}

// This struct doesn't use lifetimes to keep the code simple.
// You can try to use `&str` instead of `String`
// to avoid unnecessary allocations. 👍
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
        let result = match self.binary.as_ref() {
            "cd" => {
                let dir = self.args.get(0)?;
                let dir = std::path::PathBuf::from(dir);
                builtins::Cd::new(dir).run()
            }
            "exit" => {
                let status = match self.args.get(0) {
                    Some(status) => status.parse().unwrap_or(0),
                    None => 0,
                };
                builtins::Exit::new(status).run()
            }
            "history" => builtins::History::new().run(),
            _ => self.run_external(),
        };

        match result {
            Ok(output) => {
                match output {
                    Some(output) => {
                        // Print stdout
                        std::io::stdout().write_all(&output.stdout).unwrap();

                        // Print stderr
                        std::io::stderr().write_all(&output.stderr).unwrap();
                    }
                    None => {}
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }

        None
    }

    fn run_external(self) -> Result<Option<Output>> {
        let child = Command::new(self.binary).args(self.args).spawn()?;
        let output = child.wait_with_output().expect("command wasn't running");
        Ok(Some(output))
    }
}

fn main() {
    let history = builtins::History::new();
    loop {
        show_prompt();
        let line = read_line();
        history.add(&line.trim()).expect("Cannot open history file");
        let chains = chains_from_line(line);
        for chain in chains {
            chain.run();
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
