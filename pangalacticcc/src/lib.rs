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
pub fn answer_how_much(numeral_mapping: &HashMap<String, String>, question: &String) -> String {
    //pish tegj glob glob is 42
    unimplemented!()
}

pub fn answer_how_many(
    numeral_mapping: &HashMap<String, String>,
    unit_mapping: &HashMap<String, u32>,
    question: &String,
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
