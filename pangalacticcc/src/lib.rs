pub mod roman;
pub mod textprocessing;

use crate::roman::Roman;
use crate::textprocessing::{
    is_numeral_info, is_question_how_many_credits, is_question_how_much, is_unit_info,
    numerals_to_roman,
};

use clap::{Arg, Command};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, Read};

type PccResult<T> = Result<T, Box<dyn Error>>;

const DEFAULT_RESPONSE: &str = "I have no idea what you are talking about";

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
    let mut reader = open(&config.path)?;
    let mut buff = String::new();

    // todo: this assumes the input is of manageable size
    reader.read_to_string(&mut buff)?;

    // strip whitespace from start end end of sentences
    let contents = buff
        .split('\n')
        .into_iter()
        .map(|x| x.trim_end().trim_start())
        .collect::<Vec<_>>();

    // collect statements about alien numerals
    let numeral_info = contents
        .iter()
        .filter(|x| is_numeral_info(x))
        .collect::<Vec<_>>();

    // collect statements about alien units
    let unit_info = contents
        .iter()
        .filter(|x| is_unit_info(x))
        .collect::<Vec<_>>();

    // collect questions how much is $sequence_of_alien_numerals
    let how_much_questions = contents
        .iter()
        .filter(|x| is_question_how_much(x))
        .collect::<Vec<_>>();

    // collect questions how many credits is $sequence_of_alien_numerals $alien_unit
    let how_many_credits_questions = contents
        .iter()
        .filter(|x| is_question_how_many_credits(x))
        .collect::<Vec<_>>();

    // collect anything else that doesn't match the other patterns
    let uncategorized = contents
        .iter()
        .filter(|x| {
            !is_numeral_info(x)
                && !is_unit_info(x)
                && !is_question_how_much(x)
                && !is_question_how_many_credits(x)
                && !x.is_empty()
        })
        .collect::<Vec<_>>();

    // init and populate alien numerals -> roman numerals mapping
    let mut numeral_mapping: HashMap<String, char> = HashMap::new();
    // todo:refactor
    for x in numeral_info {
        if let Some((k, v)) = numerals_to_roman(x) {
            numeral_mapping.insert(k, v.parse().unwrap());
        }
    }

    let mut unit_mapping: HashMap<String, f64> = HashMap::new();
    for sentence in unit_info {
        if let Some((k, v)) =
            textprocessing::extract_unit_values_from_sentence(&numeral_mapping, sentence)
        {
            unit_mapping.insert(k, v);
        }
    }
    //println!("UNITS {:#?}", unit_mapping);

    //todo: is order important?
    for q in how_much_questions {
        println!("{}", answer_how_much(&numeral_mapping, q));
    }
    //todo: is order important?
    for q in how_many_credits_questions {
        println!(
            "{}",
            answer_how_many_credits(&numeral_mapping, &unit_mapping, q)
        );
    }
    // todo: uncategorized
    for _ in uncategorized {
        println!("{}", DEFAULT_RESPONSE);
    }

    // outline
    // - extract statements and questions [x]
    // - convert numerals from input to roman numerals [x]
    // - roman numerals -> values as decimal numbers in arabic numerals [x]
    //   -> answering questions "how much is $amount" possible [x]
    // - extract units from input [x]
    // - calculate conversion rate 1 $unit <-> N Credits [x]
    //   -> answering questions "how many Credits is $amount $unit ?" possible [x]
    // check for invalid inputs

    Ok(())
}

pub fn answer_how_much(numeral_mapping: &HashMap<String, char>, question: &str) -> String {
    // how much is pish tegj glob glob ?
    // -> pish tegj glob glob is 42
    //todo refactor
    let mut orig: Vec<String> = Vec::new();
    let mut numerals: Vec<String> = Vec::new();
    for word in question.split(' ') {
        if let Some(value) = numeral_mapping.get(word) {
            //todo: better error handling
            numerals.push(
                value
                    .to_string()
                    .parse::<Roman>()
                    .expect("something went wrong while parsing")
                    .repr,
            );
            orig.push(word.to_string());
        }
    }

    //println!("DEBUG: {:?} -> {:?}", orig, numerals);
    if let Ok(result) = numerals.join("").parse::<Roman>() {
        format!("{} is {}", orig.join(" "), result.value)
    } else {
        "I have no idea what you are talking about".to_string() //todo err
    }
}

pub fn answer_how_many_credits(
    numeral_mapping: &HashMap<String, char>,
    unit_mapping: &HashMap<String, f64>,
    question: &str,
) -> String {
    // how many Credits is glob prok Iron ?
    // -> glob prok Iron is 782 Credits

    // todo refactor
    let amount_unit = question
        .split("how many Credits is ")
        .into_iter()
        .filter_map(|element| element.trim_start().trim_end().strip_suffix('?'))
        .map(|x| x.trim_end())
        .collect::<Vec<_>>();

    let mut amount = amount_unit[0].split(' ').collect::<Vec<_>>();
    let unit = &amount.pop().unwrap();

    let roman_number = amount
        .iter()
        .filter_map(|x| numeral_mapping.get(*x))
        .collect::<String>(); // todo

    if let Some(value) = unit_mapping.get(*unit) {
        if let Ok(amount_parsed) = roman_number.parse::<Roman>() {
            return format!(
                "{} {} is {} Credits",
                amount.join(" "),
                unit,
                amount_parsed.value as f64 * value
            );
        }
    }
    "I have no idea what you are talking about".to_string() //todo err
}

pub fn open(path: &str) -> PccResult<Box<dyn BufRead>> {
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
    use crate::roman::ParseRomanNumeralError;

    #[test]
    fn test_answer_how_much_example_42() {
        let mut hm: HashMap<String, char> = HashMap::new();
        hm.insert("glob".to_string(), 'I');
        hm.insert("prok".to_string(), 'V');
        hm.insert("tegj".to_string(), 'L');
        hm.insert("pish".to_string(), 'X');
        let question = "how much is pish tegj glob glob ?";
        let expected = "pish tegj glob glob is 42";
        let result = answer_how_much(&hm, question);
        assert_eq!(expected, result)
    }

    #[test]
    fn test_answer_how_much_bla_8() {
        let mut hm: HashMap<String, char> = HashMap::new();
        hm.insert("bla".to_string(), 'I');
        hm.insert("blub".to_string(), 'V');
        hm.insert("blubber".to_string(), 'L');
        let question = "how much is blub bla bla bla ?";
        let expected = "blub bla bla bla is 8";
        let result = answer_how_much(&hm, question);
        assert_eq!(expected, result)
    }

    #[test]
    fn test_answer_how_much_bla_invalid() {
        let mut hm: HashMap<String, char> = HashMap::new();
        hm.insert("bla".to_string(), 'I');
        hm.insert("blub".to_string(), 'V');
        hm.insert("blubber".to_string(), 'L');
        let question = "how much is blub blubber ?"; // VL -> ParseRomanNumeralError
        let expected = "I have no idea what you are talking about";
        let result = answer_how_much(&hm, question);
        assert_eq!(expected, result)
    }

    #[test]
    fn test_answer_how_many_example_silver_86() {
        let mut nm: HashMap<String, char> = HashMap::new();
        nm.insert("glob".to_string(), 'I');
        nm.insert("prok".to_string(), 'V');
        let mut um: HashMap<String, f64> = HashMap::new();
        um.insert("Silver".to_string(), 21.5);
        let question = "how many Credits is glob prok Silver ?";
        let expected = "glob prok Silver is 86 Credits";
        let result = answer_how_many_credits(&nm, &um, question);
        assert_eq!(expected, result)
    }

    #[test]
    fn test_answer_how_many_example_gold_57800() {
        let mut nm: HashMap<String, char> = HashMap::new();
        nm.insert("glob".to_string(), 'I');
        nm.insert("prok".to_string(), 'V');
        let mut um: HashMap<String, f64> = HashMap::new();
        um.insert("Gold".to_string(), 14450.0);
        let question = "how many Credits is glob prok Gold ?";
        let expected = "glob prok Gold is 57800 Credits";
        let result = answer_how_many_credits(&nm, &um, question);
        assert_eq!(expected, result)
    }

    #[test]
    fn test_answer_how_many_example_iron_782() {
        let mut nm: HashMap<String, char> = HashMap::new();
        nm.insert("glob".to_string(), 'I');
        nm.insert("prok".to_string(), 'V');
        let mut um: HashMap<String, f64> = HashMap::new();
        um.insert("Iron".to_string(), 195.5);
        let question = "how many Credits is glob prok Iron ?";
        let expected = "glob prok Iron is 782 Credits";
        let result = answer_how_many_credits(&nm, &um, question);
        assert_eq!(expected, result)
    }
}
