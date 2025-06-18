use std::process::Command;

use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};

#[test]
fn start_execution_fails_for_unsupported_file_extension() {
    let mut cmd = Command::cargo_bin("interpreter-rs").unwrap();
    cmd.arg("some/path/to/file.txt");
    let output = cmd.output().expect("fail");

    cmd.assert().failure();
    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");

    assert_eq!(stderr, "Required '.lox' file, file not supported!\n");
}

#[test]
fn start_execution_passes_for_supported_file_extension() {
    let mut cmd = Command::cargo_bin("interpreter-rs").unwrap();
    cmd.arg("tests/inputs/test.lox");
    let output = cmd.output().expect("fail");

    cmd.assert().success();
    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");

    assert_eq!(stderr, "");
}

#[test]
fn start_execution_without_args_passes() {
    let mut cmd = Command::cargo_bin("interpreter-rs").unwrap();
    let output = cmd.output().expect("fail");

    cmd.assert().success();
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");

    assert_eq!(stdout, "> ");
    assert_eq!(stderr, "");
}
