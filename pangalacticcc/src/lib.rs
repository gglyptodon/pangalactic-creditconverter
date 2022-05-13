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

    // init and populate alien numerals -> roman numerals mapping
    let mut numeral_mapping: HashMap<String, char> = HashMap::new();
    // todo:refactor
    for x in numeral_info {
        if let Some((k, v)) = numerals_to_roman(x) {
            numeral_mapping.insert(k, v.parse().unwrap());
        }
    }

    // outline
    // - extract statements and questions [x]
    // - convert numerals from input to roman numerals [x]
    // - roman numerals -> values as decimal numbers in arabic numerals [x]
    //   -> answering questions "how much is $amount" possible [x]
    // - extract units from input [x]
    // - calculate conversion rate 1 $unit <-> N Credits [x]
    //   -> answering questions "how many Credits is $amount $unit ?" possible
    // check for invalid inputs

    Ok(())
}
pub fn answer_how_much(numeral_mapping: &HashMap<String, char>, question: &str) -> String {
    // how much is pish tegj glob glob ?
    // -> pish tegj glob glob is 42
    //todo refactor
    let mut orig: Vec<String> = Vec::new();
    let mut numerals: Vec<String> = Vec::new();
    for word in question.split(" ") {
        match numeral_mapping.get(word) {
            //todo: better error handling
            Some(value) => {
                numerals.push(
                    value
                        .to_string()
                        .parse::<Roman>()
                        .expect("something went wrong while parsing")
                        .repr,
                );
                orig.push(word.to_string())
            }
            None => {} //todo
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
    //let mut orig: Vec<String> = Vec::new();
    //let mut numerals: Vec<String> = Vec::new();
    //for word in question.split("how many Credits is ") {

    // todo refactor
    let mut amount_unit = question
        .split("how many Credits is ")
        .into_iter()
        .map(|element| element.trim_start().trim_end().strip_suffix("?"))
        .filter_map(|x| x)
        .map(|x| x.trim_end())
        .collect::<Vec<_>>();


    let mut amount = amount_unit[0].split(" ").collect::<Vec<_>>();
    let unit = &amount.pop().unwrap();

    let roman_number = amount.iter().filter_map(|x|numeral_mapping.get(*x)).collect::<String>(); // todo
    println!("{:?}, {:?} {:?} units: {:?}", amount, roman_number, numeral_mapping, unit_mapping);
    if let Some(value) = unit_mapping.get(*unit){
        if let Ok(amount_parsed) = roman_number.parse::<Roman>(){
            return  format!("{} {} is {} Credits", amount.join(" "), unit,amount_parsed.value as f64 *value )
        }

    }
    "I have no idea what you are talking about".to_string() //todo err
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
    fn test_answer_how_many_example_86() {
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
}
