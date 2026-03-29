use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("ch02").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("ch02").unwrap();
    cmd.arg("hello").assert().success();
    Ok(())
}

fn run(args: &[&str], expected: &'static str) -> TestResult {
    Command::cargo_bin("ch02")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn take_one_word() -> TestResult {
    run(&["hello there"], "hello there\n")
}

#[test]
fn take_two_words() -> TestResult {
    run(&["hello", "there"], "hello there\n")
}

#[test]
fn take_one_word_with_omit_newline() -> TestResult {
    run(&["hello there", "-n"], "hello there")
}

#[test]
fn take_two_words_with_omit_newline() -> TestResult {
    run(&["-n", "hello", "there"], "hello there")
}
