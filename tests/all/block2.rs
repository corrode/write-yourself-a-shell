use std::time::Duration;

use crate::utils::ShellRunner;

#[test]
fn shell_runs_pwd_twice() {
    let output = ShellRunner::new()
        // TODO: this doesn't work if `;` isn't separated by ' '
        .with_stdin("pwd ; pwd")
        .example("block2")
        .kill_after(Duration::from_secs(1))
        .run();

    let stdout_str = String::from_utf8(output.stdout).unwrap();
    let curr_dir_path = std::env::current_dir().unwrap();
    let curr_dir = curr_dir_path.to_str().unwrap();
    assert_eq!(stdout_str, format!("{curr_dir}\n{curr_dir}\n"));
}

#[test]
fn shell_understands_and_operator() {
    let output = ShellRunner::new()
        .with_stdin("echo hello && echo world")
        .example("block2")
        .kill_after(Duration::from_secs(1))
        .run();

    let stdout_str = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout_str, "hello\nworld\n");
}

#[test]
fn shell_understands_or_operator() {
    let output = ShellRunner::new()
        .with_stdin("echo hello || echo world")
        .example("block2")
        .kill_after(Duration::from_secs(1))
        .run();

    let stdout_str = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout_str, "hello\n");
}
