use assert_cmd::Command;
use predicates::prelude::*;
use rand::{RngExt, distr::Alphanumeric};
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PRG: &str = "ch03";
const EMPTY: &str = "tests/files/empty.txt";
const FOX: &str = "tests/files/fox.txt";
const SPIDERS: &str = "tests/files/spiders.txt";
const BUSTLE: &str = "tests/files/bustle.txt";
#[test]
fn a() {}

fn gen_bad_file() -> String {
    loop {
        let filename = rand::rng()
            .sample_iter(Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .arg(bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], EMPTY)
}

#[test]
fn fox() -> TestResult {
    run(&[FOX], FOX)
}

#[test]
fn spiders() -> TestResult {
    run(&[SPIDERS], SPIDERS)
}

#[test]
fn bustle() -> TestResult {
    run(&[BUSTLE], BUSTLE)
}
