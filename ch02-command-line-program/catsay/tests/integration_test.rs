// https://doc.rust-lang.org/std/process/struct.Command.html
// A process builder, providing fine-grained control over how a new process should be spawned.
use std::process::Command; // run programs
// https://docs.rs/assert_cmd/latest/assert_cmd/
// assert_cmd aims to simplify the process for doing integration testing of CLIs, including:
// Finding your crate’s binary to test
// Assert on the result of your program’s run.
use assert_cmd::prelude::*; // add methods on commands

#[test]
fn run_with_defaults() {
    Command::cargo_bin("catsay")
        .expect("binary exits")
        .assert()
        .success();
}