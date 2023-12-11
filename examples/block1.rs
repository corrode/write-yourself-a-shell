use std::{
    io,
    io::Write, // <--- bring flush() into scope
    process::Command,
};

#[derive(PartialEq, Debug)]
struct Cmd<'a> {
    binary: &'a str,
    args: Vec<&'a str>,
}

impl<'a> Cmd<'a> {
    fn from_line(line: &'a str) -> Option<Self> {
        let mut parts = line.split_whitespace();
        parts.next().map(|binary| Cmd {
            binary,
            args: parts.collect(),
        })
    }

    fn run(self) {
        match Command::new(self.binary).args(self.args).spawn() {
            Ok(mut child) => {
                child.wait().expect("command wasn't running");
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }
}

fn main() {
    loop {
        show_prompt();
        let line = read_line();
        if let Some(command) = Cmd::from_line(&line) {
            command.run();
        }
    }
}

fn show_prompt() {
    print!("> ");
    // Flush stoud to ensure the prompt is displayed.
    io::stdout().flush().expect("can't flush stdout");
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
        assert_eq!(Cmd::from_line(""), None);
    }

    #[test]
    fn cmd_with_no_args_is_parsed() {
        assert_eq!(
            Cmd::from_line("ls"),
            Some(Cmd {
                binary: "ls",
                args: vec![]
            })
        );
    }

    #[test]
    fn cmd_with_args_is_parsed() {
        assert_eq!(
            Cmd::from_line("ls -l"),
            Some(Cmd {
                binary: "ls",
                args: vec!["-l"]
            })
        );
    }
}
