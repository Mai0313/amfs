//! Integration tests for the `amfs` binary.
//!
//! These drive the compiled binary the way a user would, so they cover argument
//! parsing and the process exit code, which the unit tests in `src/cli.rs`
//! cannot reach.

use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_amfs");

#[test]
fn help_lists_every_subcommand() {
    let output = Command::new(BIN)
        .arg("--help")
        .output()
        .expect("amfs --help should run");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("--help should be utf-8");
    for command in ["add", "search", "get", "list", "update", "delete"] {
        assert!(stdout.contains(command), "`{command}` missing from --help");
    }
}

#[test]
fn version_is_reported() {
    let output = Command::new(BIN)
        .arg("--version")
        .output()
        .expect("amfs --version should run");

    assert!(output.status.success());
    assert!(!String::from_utf8_lossy(&output.stdout).trim().is_empty());
}

#[test]
fn unimplemented_subcommand_exits_non_zero() {
    let output = Command::new(BIN)
        .args(["add", "remember this"])
        .output()
        .expect("amfs add should run");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("not implemented"));
}

#[test]
fn unknown_subcommand_is_rejected() {
    let output = Command::new(BIN)
        .arg("summon")
        .output()
        .expect("amfs summon should run");

    assert!(!output.status.success());
}
