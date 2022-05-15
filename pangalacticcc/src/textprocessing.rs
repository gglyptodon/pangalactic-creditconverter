use crate::Roman;
use regex::Regex;
use std::collections::HashMap;

/// Returns true if questions asks how much is
/// # Example
/// ```
/// use pangalacticcc::textprocessing::is_question_how_much;
/// assert_eq!(is_question_how_much("How much is da Fish ?"), true)
/// ```
pub fn is_question_how_much(sentence: &str) -> bool {
    // example: how much is pish tegj glob glob ?
    sentence.to_lowercase().starts_with("how much is") && sentence.ends_with('?')
}

/// Returns true if questions asks for how many credits, and an alien numeral can be extracted
/// # Example
/// ```
/// use pangalacticcc::textprocessing::is_question_how_many_credits;
/// assert_eq!(is_question_how_many_credits("how many Credits is glob prok Silver ?"), true);
/// assert_eq!(is_question_how_many_credits("How many Credits is da da da Fish ?"), true);
/// ```

pub fn is_question_how_many_credits(sentence: &str) -> bool {
    sentence.to_lowercase().starts_with("how many credits is") && sentence.ends_with('?')
}

/// Returns true if sentence is statement about units, and a number of Credits can be extracted
/// # Example
/// ```
/// use pangalacticcc::textprocessing::is_unit_info;
/// assert_eq!(is_unit_info("glob prok Gold is 57800 Credits"), true);
/// assert_eq!(is_unit_info("Da da da Fish is 3 Credits"), true);
/// assert_eq!(is_unit_info("Da da da Fish is three Credits"), false);

/// ```
pub fn is_unit_info(sentence: &str) -> bool {
    extract_units_from_sentence(sentence) != None
}

/// Returns true if sentence is statement about numerals
/// # Example
/// ```
/// use pangalacticcc::textprocessing::is_numeral_info;
/// assert_eq!(is_numeral_info("da is I"), true);
/// assert_eq!(is_numeral_info("pish is X"), true);
/// assert_eq!(is_numeral_info("pish is IX"), false);
/// assert_eq!(is_numeral_info("pish is A"), false);
/// ```
pub fn is_numeral_info(sentence: &str) -> bool {
    if sentence.is_empty() {
        return false;
    }
    numerals_to_roman(sentence) != None
}

/// Returns a unit extracted from a sentence about unit info or None if extraction failed.
/// Sentences are expected to also have an amount stated directly before the unit.
/// # Example
/// ```
/// use pangalacticcc::textprocessing::extract_units_from_sentence;
/// assert_eq!(extract_units_from_sentence("glob prok Iron is 782 Credits"),Some("Iron".to_string()));
/// assert_eq!(extract_units_from_sentence("Iron is 10 Credits"), None);
/// ```
pub fn extract_units_from_sentence(sentence: &str) -> Option<String> {
    // assuming Credits is agreed upon
    let unit_regex = Regex::new(r"^([\w ]+) is (\d+) Credits$").unwrap();
    if let Some(captures) = unit_regex.captures(sentence.trim_start().trim_end()) {
        let result = captures
            .iter()
            .map(|m| m.unwrap().as_str().to_string())
            .collect::<Vec<_>>();
        // capture group 0 is always entire match,
        // group 1: $amount $unit, group 2: $amount_arabic_numerals
        if result.len() != 3 {
            return None;
        }
        let amount_unit = result.get(1).unwrap().split(' ').collect::<Vec<_>>();
        // needs at least one numeral before unit
        if amount_unit.len() < 2 {
            return None;
        }
        let unit = amount_unit.last().unwrap();
        return Some(unit.to_string());
    }
    None
}

/// Returns a amount extracted from a sentence with numerals as integer or None if extraction failed.
/// Sentences are expected to also have an amount stated directly before the unit.
/// # Example
/// ```
/// use std::collections::HashMap;
/// use pangalacticcc::textprocessing::extract_amounts_from_sentence;
/// let mut nm:HashMap<String,char> = HashMap::new();
/// nm.insert(String::from("glob"), 'I');
/// nm.insert("prok".to_string(), 'V');
/// assert_eq!(extract_amounts_from_sentence(&nm,"glob prok Iron is 782 Credits"), Some(4));
/// assert_eq!(extract_amounts_from_sentence(&nm,"glob glob Fish is 2 Credits"),Some(2));
/// ```
pub fn extract_amounts_from_sentence(
    numeral_map: &HashMap<String, char>,
    sentence: &str,
) -> Option<i32> {
    // assuming Credits is agreed upon
    // example input: glob prok Iron is 782 Credits
    let unit_regex = Regex::new(r"^([\w ]+) is (\d+) Credits$").unwrap();
    if let Some(captures) = unit_regex.captures(sentence.trim_start().trim_end()) {
        let result = captures
            .iter()
            .map(|m| m.unwrap().as_str().to_string())
            .collect::<Vec<_>>();
        // capture group 0 is always entire match,
        // group 1: $amount $unit, group 2: $amount_arabic_numerals
        if result.len() != 3 {
            return None;
        }
        let mut amount = result.get(1).unwrap().split(' ').collect::<Vec<_>>();
        amount.pop(); //discard last element which should be the unit

        let mapped = amount
            .iter()
            .filter_map(|x| numeral_map.get(*x))
            .collect::<Vec<_>>();
        // not all could be mapped -> return None
        if mapped.len() != amount.len() {
            return None;
        } else {
            let mut result = String::new();
            for n in mapped {
                result.push(*n);
            }
            return if let Ok(roman) = result.parse::<Roman>() {
                Some(roman.get_value())
            } else {
                None
            };
        }
    }
    None
}
/// Returns a amount of Credits extracted from a sentence or None if extraction failed.
/// # Example
/// ```
/// use std::collections::HashMap;
/// use pangalacticcc::textprocessing::{extract_amount_credits_from_sentence};
/// assert_eq!(extract_amount_credits_from_sentence("glob prok Iron is 782 Credits"), Some(782));
/// assert_eq!(extract_amount_credits_from_sentence("glob glob Fish is 2 Credits"), Some(2));
/// ```
pub fn extract_amount_credits_from_sentence(sentence: &str) -> Option<i32> {
    let unit_regex = Regex::new(r"^([\w ]+) is (\d+) Credits$").unwrap();
    if let Some(captures) = unit_regex.captures(sentence) {
        let result = captures
            .iter()
            .map(|m| m.unwrap().as_str().to_string())
            .collect::<Vec<_>>();
        // capture group 0 is always entire match,
        // group 1: $amount $unit, group 2: $amount_arabic_numerals
        if result.len() != 3 {
            return None;
        }
        //println!("DEBUG: {:?}", result);
        if let Some(num_credits) = result.get(2) {
            if let Ok(result) = num_credits.parse::<i32>() {
                return Some(result);
            }
        }
    }
    None
}

/// Returns credit conversion rate for unit extracted from a sentence with numerals as (unit: String, value: f64) tuple or None if extraction failed.
/// Sentences are expected to have an amount stated directly before the unit.
/// # Example
/// ```
/// use std::collections::HashMap;
/// use pangalacticcc::textprocessing::{extract_unit_values_from_sentence};
/// let mut nm:HashMap<String,char> = HashMap::new();
/// nm.insert(String::from("glob"), 'I');
/// nm.insert("prok".to_string(), 'V');
/// assert_eq!(extract_unit_values_from_sentence(
///     &nm,"glob prok Iron is 782 Credits"),Some(("Iron".to_string(),195.5))
/// );
/// assert_eq!(extract_unit_values_from_sentence(
///     &nm,"glob glob Fish is 2 Credits"), Some(("Fish".to_string(),1.0))
/// );
/// ```
pub fn extract_unit_values_from_sentence(
    numeral_map: &HashMap<String, char>,
    sentence: &str,
) -> Option<(String, f64)> {
    if let Some(amount) = extract_amounts_from_sentence(numeral_map, sentence) {
        if let Some(unit) = extract_units_from_sentence(sentence) {
            if let Some(num_credits) = extract_amount_credits_from_sentence(sentence) {
                return Some((unit, num_credits as f64 / amount as f64));
            }
        }
    }
    None
}

/// Returns (alien_numeral, roman_numeral) tuple from a sentence with numerals
/// or None if extraction failed.
/// # Example
/// ```
/// use std::collections::HashMap;
/// use pangalacticcc::textprocessing::{numerals_to_roman};
/// assert_eq!(numerals_to_roman("glob is I"), Some(("glob".to_string(), "I".to_string())));
/// assert_eq!(numerals_to_roman("pish is X"), Some(("pish".to_string(), "X".to_string())));
/// assert_eq!(numerals_to_roman("pish is A"), None);
/// ```
pub fn numerals_to_roman(sentence: &str) -> Option<(String, String)> {
    let numeral_regex = Regex::new(r"^(\w+) is ([IVXLCDM])$").unwrap();
    if let Some(mapping) = numeral_regex.captures(sentence) {
        let result = mapping
            .iter()
            .map(|m| m.unwrap().as_str().to_string())
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

#[cfg(test)]
mod tests {
    use super::*;

    const GLOB_I: &str = "glob is I";
    const PROK_V: &str = "prok is V";
    const PISH_X: &str = "pish is X";
    const TEGJ_L: &str = "tegj is L";

    const FULL_EXAMPLE: &[&str] = &[
        "glob is I",
        "prok is V",
        "pish is X",
        "tegj is L",
        "glob glob Silver is 34 Credits",
        "glob prok Gold is 57800 Credits",
        "pish pish Iron is 3910 Credits",
        "how much is pish tegj glob glob ?",
        "how many Credits is glob prok Silver ?",
        "how many Credits is glob prok Gold ?",
        "how many Credits is glob prok Iron ?",
        "",
        "how much wood could a woodchuck chuck if a woodchuck could chuck wood ?",
    ];

    fn create_testmap() -> HashMap<String, char> {
        let mut map = HashMap::new();
        map.insert("tegj".to_string(), 'L');
        map.insert("pish".to_string(), 'X');
        map.insert("prok".to_string(), 'V');
        map.insert("glob".to_string(), 'I');
        map
    }

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

    #[test]
    fn test_simple_sentence_to_value_glob() {
        let mapping = numerals_to_roman(GLOB_I).unwrap();
        let r: Roman = mapping.1.parse().unwrap();
        //println!("{r}");
        assert_eq!(r.get_value(), 1)
    }

    #[test]
    fn test_simple_sentence_to_value_prok() {
        let mapping = numerals_to_roman(PROK_V).unwrap();
        let r: Roman = mapping.1.parse().unwrap();
        //println!("{r}");
        assert_eq!(r.get_value(), 5)
    }

    #[test]
    fn test_simple_sentence_to_value_pish() {
        let mapping = numerals_to_roman(PISH_X).unwrap();
        let r: Roman = mapping.1.parse().unwrap();
        //println!("{r}");
        assert_eq!(r.get_value(), 10)
    }

    #[test]
    fn test_simple_sentence_to_value_tegj() {
        let mapping = numerals_to_roman(TEGJ_L).unwrap();
        let r: Roman = mapping.1.parse().unwrap();
        //println!("{r}");
        assert_eq!(r.get_value(), 50)
    }

    #[test]
    #[should_panic]
    fn test_simple_sentence_to_value_malformed() {
        let mapping = numerals_to_roman("bla is ?").unwrap();
        let r: Roman = mapping.1.parse().unwrap();
        //println!("{r}");
        assert_eq!(r.get_value(), 50)
    }

    #[test]
    fn test_sentence_is_question_how_much() {
        let mut expected = vec![false; 13];
        expected[7] = true;
        let results: Vec<bool> = FULL_EXAMPLE
            .iter()
            .map(|x| is_question_how_much(x))
            .collect();
        assert_eq!(expected, results)
    }

    #[test]
    fn test_sentence_is_question_how_many_credits() {
        let mut expected = vec![false; 13];
        expected[8] = true;
        expected[9] = true;
        expected[10] = true;

        let results: Vec<bool> = FULL_EXAMPLE
            .iter()
            .map(|x| is_question_how_many_credits(x))
            .collect();
        assert_eq!(expected, results)
    }

    #[test]
    fn test_sentence_is_numeral_info() {
        let mut expected = vec![false; 13];
        expected[0] = true;
        expected[1] = true;
        expected[2] = true;
        expected[3] = true;
        let results: Vec<bool> = FULL_EXAMPLE.iter().map(|x| is_numeral_info(x)).collect();
        assert_eq!(expected, results)
    }

    #[test]
    fn test_sentence_is_unit_info() {
        let mut expected = vec![false; 13];
        expected[4] = true;
        expected[5] = true;
        expected[6] = true;
        let results: Vec<bool> = FULL_EXAMPLE.iter().map(|x| is_unit_info(x)).collect();
        assert_eq!(expected, results)
    }

    #[test]
    fn test_extract_unit_gold() {
        let gold_unit = "glob prok Gold is 57800 Credits";
        let expected = "Gold".to_string();
        let result = extract_units_from_sentence(gold_unit);
        assert_eq!(Some(expected), result)
    }

    #[test]
    fn test_extract_unit_iron() {
        let iron_unit = "pish pish Iron is 3910 Credits";
        let expected = "Iron".to_string();
        let result = extract_units_from_sentence(iron_unit);
        assert_eq!(Some(expected), result)
    }

    #[test]
    fn test_extract_unit_silver() {
        let silver_unit = "glob glob Silver is 34 Credits";
        let expected = "Silver".to_string();
        let result = extract_units_from_sentence(silver_unit);
        assert_eq!(Some(expected), result)
    }

    #[test]
    fn test_extract_unit_gold_none() {
        let expected = None;
        let result = extract_units_from_sentence(GLOB_I);
        assert_eq!(expected, result)
    }

    #[test]
    fn test_extract_unit_val_gold() {
        let gold_unit = "glob prok Gold is 57800 Credits";
        let testmap = create_testmap();
        let expected = ("Gold".to_string(), 57800.0 / 4.0);
        let result = extract_unit_values_from_sentence(&testmap, gold_unit);
        assert_eq!(Some(expected), result)
    }

    #[test]
    fn test_extract_unit_val_iron() {
        let iron_unit = "pish pish Iron is 3910 Credits";
        let testmap = create_testmap();
        let expected = ("Iron".to_string(), 3910.0 / 20.0);
        let result = extract_unit_values_from_sentence(&testmap, iron_unit);
        assert_eq!(Some(expected), result)
    }

    #[test]
    fn test_extract_unit_values_silver() {
        let silver_unit = "glob glob Silver is 34 Credits";
        let testmap = create_testmap();
        let expected = ("Silver".to_string(), 34.0 / 2.0);
        let result = extract_unit_values_from_sentence(&testmap, silver_unit);
        assert_eq!(Some(expected), result)
    }

    #[test]
    fn test_extract_amount_gold() {
        let gold_unit = "glob prok Gold is 57800 Credits";
        let expected = 4;
        let testmap = create_testmap();
        let result = extract_amounts_from_sentence(&testmap, gold_unit);
        assert_eq!(Some(expected), result)
    }
    #[test]

    fn test_extract_amount_silver() {
        let silver_unit = "glob glob Silver is 34 Credits";
        let expected = 2;
        let testmap = create_testmap();
        let result = extract_amounts_from_sentence(&testmap, silver_unit);
        assert_eq!(Some(expected), result)
    }

    #[test]
    fn test_extract_amount_iron() {
        let iron_unit = "pish pish Iron is 3910 Credits";
        let expected = 20;
        let testmap = create_testmap();
        let result = extract_amounts_from_sentence(&testmap, iron_unit);
        assert_eq!(Some(expected), result)
    }

    #[test]
    fn test_extract_unit_val_full_gold() {
        let gold_unit = "glob prok Gold is 57800 Credits";
        let testmap = create_testmap();
        let expected = ("Gold".to_string(), 57800.0 / 4.0);
        let result = extract_unit_values_from_sentence(&testmap, gold_unit);
        assert_eq!(Some(expected), result)
    }

    #[test]
    fn test_extract_unit_val_full_silver() {
        let silver_unit = "glob glob Silver is 34 Credits";
        let testmap = create_testmap();
        let expected = ("Silver".to_string(), 34.0 / 2.0);
        let result = extract_unit_values_from_sentence(&testmap, silver_unit);
        assert_eq!(Some(expected), result)
    }

    #[test]
    fn test_extract_unit_val_full_iron() {
        let iron_unit = "pish pish Iron is 3910 Credits";
        let expected = ("Iron".to_string(), 3910.0 / 20.0);
        let testmap = create_testmap();
        let result = extract_unit_values_from_sentence(&testmap, iron_unit);
        assert_eq!(Some(expected), result)
    }
}
