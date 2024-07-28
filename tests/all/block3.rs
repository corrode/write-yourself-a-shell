use std::time::Duration;

use crate::utils::ShellRunner;

const SHELL_TIMEOUT: Duration = Duration::from_secs(3);

#[test]
fn supports_cd_shell_builtin() {
    let output = ShellRunner::new()
        .with_stdin("pwd; cd examples; pwd")
        .example("block3")
        .kill_after(SHELL_TIMEOUT)
        .run();

    let stdout_str = String::from_utf8(output.stdout).unwrap();
    let curr_dir_path = std::env::current_dir().unwrap();
    let curr_dir = curr_dir_path.to_str().unwrap();
    assert_eq!(stdout_str, format!("{curr_dir}\n{curr_dir}/examples\n"));
}

#[test]
fn supports_relative_cd() {
    let output = ShellRunner::new()
        .with_stdin("pwd; cd examples; pwd; cd ..; pwd")
        .example("block3")
        .kill_after(SHELL_TIMEOUT)
        .run();

    let stdout_str = String::from_utf8(output.stdout).unwrap();
    let curr_dir_path = std::env::current_dir().unwrap();
    let curr_dir = curr_dir_path.to_str().unwrap();
    assert_eq!(
        stdout_str,
        format!("{curr_dir}\n{curr_dir}/examples\n{curr_dir}\n")
    );
}

#[test]
fn invalid_cd_doesnt_change_dir() {
    let output = ShellRunner::new()
        .with_stdin("pwd; cd invalid_dir; pwd")
        .example("block3")
        .kill_after(SHELL_TIMEOUT)
        .run();

    let stdout_str = String::from_utf8(output.stdout).unwrap();
    let curr_dir_path = std::env::current_dir().unwrap();
    let curr_dir = curr_dir_path.to_str().unwrap();
    assert_eq!(stdout_str, format!("{curr_dir}\n{curr_dir}\n"));
}

#[test]
fn supports_exit_builtin() {
    let output = ShellRunner::new()
        .with_stdin("exit 1")
        .example("block3")
        .kill_after(SHELL_TIMEOUT)
        .run();

    assert_eq!(output.status.code(), Some(1));
}
