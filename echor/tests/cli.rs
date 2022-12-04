use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn dies_no_args() {
    let mut command = Command::cargo_bin("echor").unwrap();
    command.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}