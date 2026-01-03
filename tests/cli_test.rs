use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn print_help() {
    let path = assert_cmd::cargo::cargo_bin!("bootleg");
    let mut bootleg = Command::new(path);
    bootleg
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Examples:"));
}
