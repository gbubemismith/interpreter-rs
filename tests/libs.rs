use std::process::Command;

use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};

#[test]
fn start_execution_fails_for_unsupported_file_extension() {
    let mut cmd = Command::cargo_bin("interpreter-rs").unwrap();
    cmd.arg("some/path/to/file.txt");
    let output = cmd.output().expect("fail");

    println!("Output:: {:?}", output);

    cmd.assert().failure();
    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");

    assert_eq!(stderr, "Required '.lox' file, file not supported!\n");
}
