use std::error::Error;

use assert_cmd::Command;
use predicates::prelude::predicate;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "catr";

#[test]
fn usage() -> TestResult {
    for flg in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flg)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }
    Ok(())
}
