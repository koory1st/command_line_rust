use std::error::Error;
use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

const COMMAND_NAME: &'static str = "echor";

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut command = Command::cargo_bin(COMMAND_NAME)?;
    command
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

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
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}

fn run(args: &[&str], outfile: &str) -> Result<(), Box<dyn Error>> {
    let expected = fs::read_to_string(outfile)?;
    let mut command = Command::cargo_bin(COMMAND_NAME)?;
    command.args(args).assert().success().stdout(expected);

    Ok(())
}
