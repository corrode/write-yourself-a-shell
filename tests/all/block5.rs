use std::time::Duration;

use crate::utils::ShellRunner;

const SHELL_TIMEOUT: Duration = Duration::from_secs(3);

#[test]
fn test_pipes() {
    let output = ShellRunner::new()
        .with_stdin("echo hello | wc -c\n")
        .example("block5")
        .kill_after(SHELL_TIMEOUT)
        .run();

    let stdout_str = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout_str, "6\n");
}
