use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn print_help() {
    let mut cmd = Command::cargo_bin("bootleg").unwrap();
    cmd.arg("--help").assert().success().stdout(predicate::str::contains("Examples:"));
}
