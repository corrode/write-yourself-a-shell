use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs, path::PathBuf, process, time::Duration};

use crate::utils::ShellRunner;

const SHELL_TIMEOUT: Duration = Duration::from_secs(3);

fn generate_temp_file_name() -> PathBuf {
    let temp_dir = env::temp_dir();
    let pid = process::id();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();
    let file_name = format!("temp_file_{}_{}", pid, timestamp);
    temp_dir.join(file_name)
}

#[test]
fn test_history() {
    // Random path to history
    let history_path = generate_temp_file_name();
    std::env::set_var("HISTORY_PATH", &history_path);

    ShellRunner::new()
        .with_stdin("echo 1\necho 2\nhistory")
        .example("block4")
        .kill_after(SHELL_TIMEOUT)
        .run();

    let history_contents = fs::read_to_string(history_path).unwrap();
    assert!(history_contents.starts_with("echo 1\necho 2\nhistory\n"))
}
