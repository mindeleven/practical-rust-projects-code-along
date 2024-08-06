// https://doc.rust-lang.org/std/process/struct.Command.html
// A process builder, providing fine-grained control over how a new process should be spawned.
// gives us a command struct that can run programs in a newly spawned process
use std::process::Command; // run programs
// https://docs.rs/assert_cmd/latest/assert_cmd/
// assert_cmd aims to simplify the process for doing integration testing of CLIs, including:
// Finding your crate’s binary to test
// Assert on the result of your program’s run.
use assert_cmd::prelude::*; // add methods on commands, traits that extend Command
// predicates allows to check STDOUT and see if it contains the expected output
use predicates::prelude::*;

#[test]
fn run_with_defaults() {
    // initializing the command
    Command::cargo_bin("catsay") // takes cargo build binary name
        .expect("binary exits")
        .assert() // if binary, produce assert struct 
        .success()// very basic assertion if we stop here
        .stdout(predicate::str::contains("Meow!")); // verifying default STDOUT
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    // example: checking if an invalid -f argument is handled correctly
    Command::cargo_bin("catsay") // takes cargo build binary name
        .expect("binary exits")
        .args(&["-f", "no/such/file.txt"]) // expected to exit with error 
        // .args(&["-f", "assets/catfile.txt"]) // file exits -> test fails 
        .assert()
        .failure(); // check if it actually fails
    Ok(())
}