use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

const COMMAND_NAME: &'static str = "echor";

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut command = Command::cargo_bin(COMMAND_NAME)?;
    command.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut command = Command::cargo_bin(COMMAND_NAME).unwrap();
    command.arg("test").assert().success();

    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile)?;
    let mut command = Command::cargo_bin(COMMAND_NAME)?;
    command.arg("Hello there").assert().success().stdout(expected);

    Ok(())
}

#[test]
fn hello2() -> TestResult {
    let outfile = "tests/expected/hello2.txt";
    let expected = fs::read_to_string(outfile)?;
    let mut command = Command::cargo_bin(COMMAND_NAME)?;
    command.args(vec!["Hello", "there"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}