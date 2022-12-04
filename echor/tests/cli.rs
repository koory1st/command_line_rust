use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

const COMMAND_NAME: &'static str = "echor";

#[test]
fn dies_no_args() {
    let mut command = Command::cargo_bin(COMMAND_NAME).unwrap();
    command.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs() {
    let mut command = Command::cargo_bin(COMMAND_NAME).unwrap();
    command.arg("test").assert().success();
}

#[test]
fn hello1() {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut command = Command::cargo_bin(COMMAND_NAME).unwrap();
    command.arg("Hello there").assert().success().stdout(expected);
}