use crate::{roman, Roman};
use regex::Regex;

pub fn is_question_how_much(sentence: &str) -> bool {
    /// example: how much is pish tegj glob glob ?
    if sentence.starts_with("how much is") && sentence.ends_with("?") {
        true
    } else {
        false
    }
}

pub fn is_question_how_many_credits(sentence: &str) -> bool {
    /// example: how many Credits is glob prok Silver ?
    if sentence.starts_with("how many Credits is") && sentence.ends_with("?") {
        true
    } else {
        false
    }
}

pub fn is_unit_info(sentence: &str) -> bool {
    //todo: make more specific
    /// example: glob prok Gold is 57800 Credits -> true
    if sentence.ends_with("Credits") {
        true
    } else {
        false
    }
}

pub fn is_numeral_info(sentence: &str) -> bool {
    /// example: pish is X -> true
    if sentence.is_empty() {
        return false;
    }
    // sentence ends on roman numeral
    if roman::ROMAN_VALUES.contains_key(&sentence.chars().last().unwrap()) {
        true
    } else {
        false
    }
}

pub fn extract_units_from_sentence(sentence: &str) -> Option<(String, String)> {
    // assuming Credits is agreed upon
    /// example input: glob prok Iron is 782 Credits
    let unit_regex = Regex::new(r"^([\w ]+) is (\d+) Credits$").unwrap();
    unimplemented!()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::roman::Roman;

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
        assert_eq!(r.value, 1)
    }
    #[test]
    fn test_simple_sentence_to_value_prok() {
        let mapping = numerals_to_roman(PROK_V).unwrap();
        let r: Roman = mapping.1.parse().unwrap();
        //println!("{r}");
        assert_eq!(r.value, 5)
    }
    #[test]
    fn test_simple_sentence_to_value_pish() {
        let mapping = numerals_to_roman(PISH_X).unwrap();
        let r: Roman = mapping.1.parse().unwrap();
        //println!("{r}");
        assert_eq!(r.value, 10)
    }

    #[test]
    fn test_simple_sentence_to_value_tegj() {
        let mapping = numerals_to_roman(TEGJ_L).unwrap();
        let r: Roman = mapping.1.parse().unwrap();
        //println!("{r}");
        assert_eq!(r.value, 50)
    }

    #[test]
    #[should_panic]
    fn test_simple_sentence_to_value_malformed() {
        let mapping = numerals_to_roman("bla is ?").unwrap();
        let r: Roman = mapping.1.parse().unwrap();
        //println!("{r}");
        assert_eq!(r.value, 50)
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
}
