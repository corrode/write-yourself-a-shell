use std::{
    io::{self, Read, Write},
    process::{Child, Command, Output, Stdio},
};

pub struct ShellRunner<'a> {
    stdin: Option<&'a str>,
}

impl<'a> ShellRunner<'a> {
    pub fn new() -> Self {
        Self { stdin: None }
    }

    pub fn with_stdin(mut self, stdin: &'a str) -> Self {
        self.stdin = Some(stdin);
        self
    }

    pub fn run_until_exit(&self) -> Output {
        let mut command = Command::new("cargo");
        command.args(["run", "--quiet"]).stdin(Stdio::piped());

        let mut child = command.spawn().unwrap();

        if let Some(stdin) = self.stdin {
            let mut child_stdin = child.stdin.take().expect("Failed to open stdin");
            let stdin = stdin.as_bytes().to_vec();
            std::thread::spawn(move || {
                child_stdin
                    .write_all(&stdin)
                    .expect("Failed to write to stdin");
            });
        }

        child.wait_with_output().unwrap()
    }

    pub fn run_and_wait_for_stdout(&self) -> Output {
        let mut command = Command::new("cargo");
        command
            .args(["run", "--example", "block1"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped());

        let child = command.spawn().unwrap();

        // if let Some(stdin) = self.stdin {
        //     let mut child_stdin = child.stdin.take().expect("Failed to open stdin");
        //     let stdin = stdin.as_bytes().to_vec();
        //     std::thread::spawn(move || {
        //         child_stdin
        //             .write_all(&stdin)
        //             .expect("Failed to write to stdin");
        //     });
        // }

        wait_with_input_output(child, self.stdin.map(|s| s.as_bytes().to_vec()))
    }
}

// Inspired by https://github.com/assert-rs/assert_cmd/blob/a909b08504ab16170f2eb7ab30b2c5b53c69ebd0/src/cmd.rs#L442
fn wait_with_input_output(mut child: Child, input: Option<Vec<u8>>) -> Output {
    let stdin = input.and_then(|i| {
        child
            .stdin
            .take()
            .map(|mut stdin| std::thread::spawn(move || stdin.write_all(&i)))
    });
    fn read<R>(mut input: R) -> std::thread::JoinHandle<io::Result<Vec<u8>>>
    where
        R: Read + Send + 'static,
    {
        std::thread::spawn(move || {
            let mut ret = Vec::new();
            input.read_to_end(&mut ret).map(|_| ret)
        })
    }
    let stdout = child.stdout.take().map(read);
    let stderr = child.stderr.take().map(read);

    // Finish writing stdin before waiting, because waiting drops stdin.
    stdin.and_then(|t| t.join().unwrap().ok());
    std::thread::sleep(std::time::Duration::from_millis(2000));
    child.kill().unwrap();
    let status = child.wait().unwrap();

    let stdout = stdout
        .and_then(|t| t.join().unwrap().ok())
        .unwrap_or_default();
    let stderr = stderr
        .and_then(|t| t.join().unwrap().ok())
        .unwrap_or_default();

    Output {
        status,
        stdout,
        stderr,
    }
}
