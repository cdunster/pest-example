use assert_cmd::prelude::*;
use predicates::prelude::predicate;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn can_parse_empty_file() {
    let file = NamedTempFile::new().expect("Failed to create temporary file");
    let mut cmd = Command::cargo_bin("pest-example").unwrap();
    cmd.arg(&file.path()).assert().success();
}

#[test]
fn can_parse_file() {
    let mut file = NamedTempFile::new().expect("Failed to create temporary file");
    write!(file, "1,2, 5\n42,12.125,5.25").unwrap();
    let mut cmd = Command::cargo_bin("pest-example").unwrap();
    cmd.arg(&file.path()).assert().success();
}

#[test]
fn can_sum_correctly() {
    let mut file = NamedTempFile::new().expect("Failed to create temporary file");
    write!(file, "1,2, 5\n42,12.125,5.25").unwrap();
    let mut cmd = Command::cargo_bin("pest-example").unwrap();
    cmd.arg(&file.path())
        .assert()
        .success()
        .stdout(predicate::eq("Sum: 67.375\n"));
}
