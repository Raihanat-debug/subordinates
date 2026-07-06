use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_sample() {
    let mut child = Command::new("cargo")
        .args(["run", "--bin", "iterative_dfs"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run iterative_dfs");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"5\n1 1 2 3\n")
        .unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.trim(), "4 1 1 0 0");
}

#[test]
fn test_single_employee() {
    let mut child = Command::new("cargo")
        .args(["run", "--bin", "iterative_dfs"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run iterative_dfs");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"1\n")
        .unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.trim(), "0");
}