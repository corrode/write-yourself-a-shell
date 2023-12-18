use std::time::Duration;

use crate::utils::ShellRunner;

#[test]
fn shell_can_run_pwd() {
    let curr_dir_path = std::env::current_dir().unwrap();
    let curr_dir = curr_dir_path.to_str().unwrap();
    let output = ShellRunner::new()
        .with_stdin("pwd\n")
        .kill_after(Duration::from_secs(1))
        .run_and_wait_for_stdout();
    assert!(!output.status.success());
    let stdout_str = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout_str, format!("{curr_dir}\n"));
}
