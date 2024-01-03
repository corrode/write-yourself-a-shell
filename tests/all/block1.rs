use std::time::Duration;

use crate::utils::ShellRunner;

#[test]
fn shell_runs_pwd() {
    let output = ShellRunner::new()
        .with_stdin("pwd")
        .example("block1")
        .kill_after(Duration::from_secs(1))
        .run();

    let stdout_str = String::from_utf8(output.stdout).unwrap();
    let curr_dir_path = std::env::current_dir().unwrap();
    let curr_dir = curr_dir_path.to_str().unwrap();
    assert_eq!(stdout_str, format!("{curr_dir}\n"));
}
