use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "pangalacticcc";

const INPUT1: &str = "tests/input/input1.txt";
const OUTPUT1: &str = "tests/expected/input1.txt.out";
const INPUT1_REORDERED: &str = "tests/input/input1_reordered.txt";
const OUTPUT1_REORDERED: &str = "tests/expected/input1_reordered.txt.out";
const INPUT2: &str = "tests/input/input2.txt";
const OUTPUT2: &str = "tests/expected/input2.txt.out";
// test boilerplate
fn gen_non_existing_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
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

// same as run but piping over stdin
fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn die_on_non_existing_file() -> TestResult {
    let non_existing = gen_non_existing_file();
    let expected = format!(".* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .arg(&non_existing)
        .assert()
        .failure()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

#[test]
fn test_input1() -> TestResult {
    run(&[INPUT1], OUTPUT1)
}

#[test]
fn test_input1_stdin() -> TestResult {
    run_stdin(INPUT1, &[], OUTPUT1)
}

#[test]
fn test_input1_reordered() -> TestResult {
    run(&[INPUT1_REORDERED], OUTPUT1_REORDERED)
}

#[test]
#[ignore]
fn test_input2() -> TestResult {
    run(&[INPUT2], OUTPUT2)
}
