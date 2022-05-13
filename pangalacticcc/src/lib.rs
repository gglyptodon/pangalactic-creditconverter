pub mod roman;
pub mod textprocessing;

use crate::roman::Roman;
use crate::textprocessing::numerals_to_roman;

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
    // - convert numerals from input to roman numerals [x]
    // - roman numerals -> values as decimal numbers in arabic numerals [x]
    //   -> answering questions "how much is $amount" possible
    // - extract units from input
    // - calculate conversion rate 1 $unit <-> N Credits
    //   -> answering questions "how many Credits is $amount $unit ?" possible
    // check for invalid inputs

    Ok(())
}
pub fn answer_how_much(numeral_mapping: &HashMap<String, char>, question: &str) -> String {
    //pish tegj glob glob is 42
    unimplemented!()
}

pub fn answer_how_many(
    numeral_mapping: &HashMap<String, char>,
    unit_mapping: &HashMap<String, u32>,
    question: &str,
) -> String {
    //glob prok Iron is 782 Credits
    unimplemented!()
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
    use crate::roman::ParseRomanNumeralError;
    use super::*;

    #[test]
    fn test_answer_how_much_example_42() {
        let mut hm: HashMap<String,char> = HashMap::new();
        hm.insert("glob".to_string(), 'I');
        hm.insert("prok".to_string(), 'V');
        hm.insert("tegj".to_string(), 'L');
        hm.insert("pish".to_string(), 'X');
        let question = "how much is pish tegj glob glob ?";
        let expected = "pish tegj glob glob is 42";
        let result = answer_how_much(&hm,question);
        assert_eq!(expected,result)


    }
    #[test]
    fn test_answer_how_much_bla_8() {
        let mut hm: HashMap<String,char> = HashMap::new();
        hm.insert("bla".to_string(), 'I');
        hm.insert("blub".to_string(), 'V');
        hm.insert("blubber".to_string(), 'L');
        let question = "how much is blub bla bla bla ?";
        let expected = "blub bla bla bla is 8";
        let result = answer_how_much(&hm,question);
        assert_eq!(expected,result)
    }

    #[test]
    fn test_answer_how_much_bla_invalid() {
        let mut hm: HashMap<String,char> = HashMap::new();
        hm.insert("bla".to_string(), 'I');
        hm.insert("blub".to_string(), 'V');
        hm.insert("blubber".to_string(), 'L');
        let question = "how much is blub blubber ?"; // VL -> ParseRomanNumeralError
        let expected = "I have no idea what you are talking about";
        let result = answer_how_much(&hm,question);
        assert_eq!(expected,result)
    }

}
