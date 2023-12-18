use std::{
    io::Write,
    process::{Child, Command, Output, Stdio},
    time::Duration,
};

pub struct ShellRunner<'a> {
    stdin: Option<&'a str>,
    kill_after: Option<Duration>,
    example: Option<&'a str>,
}

impl<'a> ShellRunner<'a> {
    pub fn new() -> Self {
        Self {
            stdin: None,
            kill_after: None,
            example: None,
        }
    }

    pub fn with_stdin(mut self, stdin: &'a str) -> Self {
        self.stdin = Some(stdin);
        self
    }

    pub fn example(mut self, example: &'a str) -> Self {
        self.example = Some(example);
        self
    }

    /// Wait duration and kill the command afterwards.
    /// Useful to test commands that don't exit on their own.
    pub fn kill_after(mut self, duration: Duration) -> Self {
        self.kill_after = Some(duration);
        self
    }

    pub fn run(&self) -> Output {
        let mut command = Command::new("cargo");
        command.arg("run");
        if let Some(example) = self.example {
            command.args(["--example", example]);
        }
        command.stdin(Stdio::piped()).stdout(Stdio::piped());

        let child = command.spawn().unwrap();

        self.wait_with_input_output(child, self.stdin.map(|s| s.as_bytes().to_vec()))
    }

    fn wait_with_input_output(&self, mut child: Child, input: Option<Vec<u8>>) -> Output {
        let stdin = input.and_then(|i| {
            child.stdin.take().map(|mut stdin| {
                std::thread::spawn(move || {
                    stdin.write_all(&i).unwrap();
                    stdin.flush().unwrap();
                })
            })
        });

        // Finish writing stdin before waiting, because waiting drops stdin.
        if let Some(t) = stdin {
            t.join().unwrap()
        }

        if let Some(duration) = self.kill_after {
            std::thread::sleep(duration);
            child.kill().unwrap();
        }

        child.wait_with_output().unwrap()
    }
}
