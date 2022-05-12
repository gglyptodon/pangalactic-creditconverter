pub mod roman;
use clap::{Arg, Command};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufRead;

type PccResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    /// path to the input file with the gathered information.
    /// If path is "-", input will be read from stdin.
    path: String,
}

pub fn get_args() -> PccResult<Config> {
    let matches = Command::new("pangalacticc")
        .about("Pangalactic Credit Converter")
        .arg(
            Arg::new("input_path")
                .allow_invalid_utf8(true)
                .value_name("FILE")
                .default_value("-")
                .help(
                    "Input file with gathered information and queries. \
                    If set to '-' or no FILE is specified, input is read from stdin.",
                ),
        )
        .get_matches();

    // we can safely unwrap here because we set a default
    let path = matches.value_of_lossy("input_path").unwrap().to_string();

    Ok(Config { path })
}

/// output is printed to stdout
pub fn run(config: Config) -> PccResult<()> {
    //unimplemented!();
    let reader = open(&config.path)?;
    for line in reader.lines() {
        println!("{:?}", line);
    }

    // outline
    // - extract statements and questions
    // - convert numerals from input to roman numerals
    // - roman numerals -> values as decimal numbers in arabic numerals
    //   -> answering questions "how much is $amount" possible
    // - extract units from input
    // - calculate conversion rate 1 $unit <-> N Credits
    //   -> answering questions "how many Credits is $amount $unit ?" possible
    // check for invalid inputs

    Ok(())
}

pub fn numerals_to_roman(sentence: &str) -> Option<(String, String)> {
    let numeral_regex = Regex::new(r"^(\w+) is ([IVXLCDM])$").unwrap();
    if let Some(mapping) = numeral_regex.captures(&sentence) {
        let result = mapping
            .iter()
            .map(|m| m.unwrap().as_str().to_string().clone())
            .collect::<Vec<_>>();
        // capture group 0 is always entire match,
        // the other two should be found in a valid sentence
        if result.len() != 3 {
            return None;
        }
        return Some((
            result.get(1).unwrap().clone(),
            result.get(2).unwrap().clone(),
        ));
    }
    None
}

pub fn open(path: &String) -> PccResult<Box<dyn BufRead>> {
    if path == "-" {
        // input is stdin
        Ok(Box::new(std::io::BufReader::new(std::io::stdin())))
    } else {
        // input read from file
        Ok(Box::new(std::io::BufReader::new(File::open(path)?)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tests_run() {
        assert_eq!(1 + 2, 3);
    }
    #[test]
    fn test_numerals_to_roman_from_example() {
        assert_eq!(
            numerals_to_roman("glob is I"),
            Some(("glob".to_string(), "I".to_string()))
        );
        assert_eq!(
            numerals_to_roman("prok is V"),
            Some(("prok".to_string(), "V".to_string()))
        );
        assert_eq!(
            numerals_to_roman("pish is X"),
            Some(("pish".to_string(), "X".to_string()))
        );
        assert_eq!(
            numerals_to_roman("tegj is L"),
            Some(("tegj".to_string(), "L".to_string()))
        )
    }
    #[test]
    fn test_numerals_to_roman_non_ascii() {
        assert_eq!(
            numerals_to_roman("五 is V"),
            Some(("五".to_string(), "V".to_string()))
        );
    }
    #[test]
    fn test_numerals_to_roman_arabic_numbers() {
        assert_eq!(
            numerals_to_roman("5 is V"),
            Some(("5".to_string(), "V".to_string()))
        );
    }
    #[test]
    fn test_numerals_to_roman_mixed() {
        assert_eq!(
            numerals_to_roman("Five五5 is V"),
            Some(("Five五5".to_string(), "V".to_string()))
        );
    }

    #[test] // todo: should that work?
    #[ignore]
    fn test_numerals_to_roman_dashes() {
        assert_eq!(
            numerals_to_roman("fi-ve is V"),
            Some(("fi-ve".to_string(), "V".to_string()))
        );
    }

    #[test] // no roman at end of sentence
    fn test_numerals_to_roman_nonroman() {
        assert_eq!(numerals_to_roman("sth is 5"), None);
    }

    #[test] // no clear roman numeral at end
    fn test_numerals_to_roman_roman_mixed() {
        assert_eq!(numerals_to_roman("sth is 5X"), None);
        assert_eq!(numerals_to_roman("sth is X5"), None);
    }

    #[test] // todo: should this work?
    fn test_numerals_to_roman_roman_multi() {
        assert_eq!(numerals_to_roman("seven is VII"), None);
    }

    #[test]
    fn test_numerals_to_roman_malformed() {
        assert_eq!(numerals_to_roman("sth completely different. X"), None);
    }
    //todo: test contradicting info
}
