//! A helpful calculator for hitchhiking merchants all across the galaxy and beyond.
//! Converts alien numerals and units to Credits.
//! Input is read from stdin or text file.
//! Output is printed to stdout.
extern crate core;

pub mod roman;
pub mod textprocessing;

use crate::roman::Roman;
use crate::textprocessing::{
    extract_unit_values_from_sentence, is_numeral_info, is_question_how_many_credits,
    is_question_how_much, is_unit_info, numerals_to_roman,
};

use crate::StatementKind::{
    HowManyQuestion, HowMuchQuestion, NumeralStatement, Uncategorized, UnitStatement,
};
use clap::{Arg, Command};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, Read};

type PccResult<T> = Result<T, Box<dyn Error>>;

const DEFAULT_RESPONSE: &str = "I have no idea what you are talking about";

/// Holds the path of the file to be processed as String.
#[derive(Debug)]
pub struct Config {
    /// path to the input file with the gathered information.
    /// If path is "-", input will be read from stdin.
    path: String,
}
#[derive(Debug, PartialEq)]
enum StatementKind {
    HowMuchQuestion,
    HowManyQuestion,
    UnitStatement,
    NumeralStatement,
    Uncategorized,
}

#[derive(Debug)]
struct InputStatement {
    kind: StatementKind,
    text: String,
}

/// Parses command line arguments
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

/// Runs the program on provided config.
/// Output is printed to stdout
pub fn run(config: Config) -> PccResult<()> {
    // outline
    // - extract statements and questions [x]
    // - convert numerals from input to roman numerals [x]
    // - roman numerals -> values as decimal numbers in arabic numerals [x]
    //   -> answering questions "how much is $amount" possible [x]
    // - extract units from input [x]
    // - calculate conversion rate 1 $unit <-> N Credits [x]
    //   -> answering questions "how many Credits is $amount $unit ?" possible [x]
    // check for invalid inputs

    let mut reader = open(&config.path)?;
    let mut buff = String::new();

    // this assumes the input is of manageable size
    reader.read_to_string(&mut buff)?;

    // strip whitespace from start end end of sentences
    let contents = buff
        .split('\n')
        .into_iter()
        .map(|x| x.trim_end().trim_start())
        .collect::<Vec<_>>();

    let mut statements: Vec<InputStatement> = Vec::new();
    for statement in &contents {
        // skip empty
        if statement.is_empty() {
            continue;
        }
        if is_numeral_info(statement) {
            // collect statements about alien numerals
            statements.push(InputStatement {
                text: statement.to_string(),
                kind: NumeralStatement,
            });
        } else if is_unit_info(statement) {
            // collect statements about alien units
            statements.push(InputStatement {
                text: statement.to_string(),
                kind: UnitStatement,
            });
        } else if is_question_how_many_credits(statement) {
            // collect questions how many credits is <sequence_of_alien_numerals> <alien_unit>
            statements.push(InputStatement {
                text: statement.to_string(),
                kind: HowManyQuestion,
            })
        } else if is_question_how_much(statement) {
            // collect questions how much is <sequence_of_alien_numerals>
            statements.push(InputStatement {
                text: statement.to_string(),
                kind: HowMuchQuestion,
            })
        } else {
            // collect anything else that doesn't match the other patterns
            statements.push(InputStatement {
                text: statement.to_string(),
                kind: Uncategorized,
            })
        }
    }

    // init and populate alien numerals -> roman numerals mapping
    let mut numeral_mapping: HashMap<String, char> = HashMap::new();
    for s in statements.iter().filter(|x| x.kind == NumeralStatement) {
        if let Some((k, v)) = numerals_to_roman(&s.text) {
            numeral_mapping.insert(k, v.parse().unwrap());
        }
    }
    // init and populate alien units -> value as float (Credits)
    let mut unit_mapping: HashMap<String, f64> = HashMap::new();
    for s in statements.iter().filter(|x| x.kind == UnitStatement) {
        if let Ok((k, v)) = extract_unit_values_from_sentence(&numeral_mapping, &s.text) {
            if let Some(old_value) = unit_mapping.get(&*k){
                if *old_value != v{
                    println!("\"{}\" has ambiguous value. Old: {}, new {}. Using new definition.", k, old_value, v)
                }
            }
            unit_mapping.insert(k, v);
        } else {
            println!("I don't understand this statement about units: {}", s.text)
        }
    }

    // answer questions
    for q in statements
        .iter()
        .filter(|x| x.kind != UnitStatement && x.kind != NumeralStatement)
    {
        if q.kind == HowMuchQuestion {
            println!("{}", answer_how_much(&numeral_mapping, &q.text));
        } else if q.kind == HowManyQuestion {
            println!(
                "{}",
                answer_how_many_credits(&numeral_mapping, &unit_mapping, &q.text)
            );
        } else {
            println!("{}", DEFAULT_RESPONSE);
        }
    }
    Ok(())
}

/// Returns response to input asking "how much is ..." as String
/// # Arguments
/// * `numeral_mapping` - Reference to HashMap mapping alien numerals to chars I,V,X,L,C,D,M
/// * `question` - Input question as string that should be answered
/// # Example
/// ```
/// use std::collections::HashMap;
/// use pangalacticcc::answer_how_much;
/// let mut nm: HashMap<String, char> = HashMap::new();
/// nm.insert("pish".to_string(), 'X');
/// nm.insert("tegj".to_string(), 'L');
/// nm.insert("glob".to_string(), 'I');
/// let q = "how much is pish tegj glob glob ?";
/// assert_eq!(answer_how_much(&nm, q),"pish tegj glob glob is 42".to_string());
/// ```
pub fn answer_how_much(numeral_mapping: &HashMap<String, char>, question: &str) -> String {
    let mut orig: Vec<String> = Vec::new();
    let mut numerals: Vec<String> = Vec::new();
    let reserved_tokens = ["?", "how", "much", "is"];

    for word in question.split(' ') {
        if numeral_mapping.get(word).is_none()
            && !reserved_tokens.contains(&word)
            && !word.is_empty()
        {
            print!("{} could not be translated. ", word)
        }
        if let Some(value) = numeral_mapping.get(word) {
            numerals.push(
                value
                    .to_string()
                    .parse::<Roman>()
                    .expect("something went wrong while parsing")
                    .get_representation(),
            );
            orig.push(word.to_string());
        }
    }

    if let Ok(result) = numerals.join("").parse::<Roman>() {
        format!("{} is {}", orig.join(" "), result.get_value())
    } else {
        format!(
            "I don't know how to interpret this number: {} -> {}",
            orig.join(" "),
            numerals.join("")
        )
    }
}

/// Returns response to input asking "how many credits is ..." as String
/// # Arguments
/// * `numeral_mapping` - Reference to HashMap mapping alien numerals to chars I,V,X,L,C,D,M
/// * `question` - Input question as string that should be answered
/// # Example
/// ```
/// use std::collections::HashMap;
/// use pangalacticcc::{answer_how_many_credits};
/// let mut nm: HashMap<String, char> = HashMap::new();
/// nm.insert("pish".to_string(), 'X');
/// nm.insert("prok".to_string(), 'V');
/// nm.insert("glob".to_string(), 'I');
/// let mut um: HashMap<String, f64> = HashMap::new();
/// um.insert("Iron".to_string(), 195.5);
/// let q = "how many Credits is glob prok Iron ?";
/// assert_eq!(answer_how_many_credits(&nm, &um, q), "glob prok Iron is 782 Credits".to_string());
/// let q2 = "how many Credits is bla prok Iron ?";
/// assert_eq!(answer_how_many_credits(&nm, &um, q2), "Not everything could be translated to roman numerals: bla prok".to_string());
/// let q3 = "how many Credits is glob prok Fish ?";
/// assert_eq!(answer_how_many_credits(&nm, &um, q3), "This unit is unkown to me: Fish".to_string());
/// ```
pub fn answer_how_many_credits(
    numeral_mapping: &HashMap<String, char>,
    unit_mapping: &HashMap<String, f64>,
    question: &str,
) -> String {
    // todo refactor
    // todo: error on incomplete mappings!
    let default = "I have no idea what you are talking about".to_string();
    let amount_unit = question
        .split("how many Credits is ")
        .into_iter()
        .filter_map(|element| element.trim_start().trim_end().strip_suffix('?'))
        .map(|x| x.trim_end())
        .collect::<Vec<_>>();

    // return default response if sentence is of different structure
    let mut amount = match amount_unit.get(0) {
        None => return default,
        Some(a) => a.split(' ').collect::<Vec<_>>(),
    };
    // split amount and unit
    let unit = &amount.pop().unwrap();

    let roman_number = amount
        .iter()
        .filter_map(|x| numeral_mapping.get(*x))
        .collect::<String>();

    // return early if alien numeral could not be converted
    if roman_number.len() != amount.len() {
        return format!(
            "Not everything could be translated to roman numerals: {}",
            amount.join(" ")
        );
    }

    if let Some(value) = unit_mapping.get(*unit) {
        if let Ok(amount_parsed) = roman_number.parse::<Roman>() {
            return format!(
                "{} {} is {} Credits",
                amount.join(" "),
                unit,
                amount_parsed.get_value() as f64 * value
            );
        }
    } else {
        // couldn't find unit in map
        return format!("This unit is unkown to me: {}", unit);
    }
    default
}

/// Returns a BufReader for `path` on success.
/// If `path` is `"-"`,  BufReader for stdin is returned,
/// otherwise BufReader for file `path` is returned.
/// # Arguments
/// * `path` -  A file name or "-".
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
        let expected = "I don't know how to interpret this number: blub blubber -> VL";
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
