use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

lazy_static! {
    static ref ROMAN_VALUES: HashMap<char, i32> = {
        let mut map = HashMap::new();
        map.insert('M', 1000);
        map.insert('D', 500);
        map.insert('C', 100);
        map.insert('L', 50);
        map.insert('X', 10);
        map.insert('V', 5);
        map.insert('I', 1);
        map
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseRomanNumeralError;

impl Display for ParseRomanNumeralError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid string for roman numeral")
    }
}

#[derive(Debug)]
pub struct Roman {
    repr: String,
    value: i32,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.repr, self.value)
    }
}

impl FromStr for Roman {
    type Err = ParseRomanNumeralError;
    /// The symbols "I", "X", "C", and "M" can be repeated three times in succession, but no more.
    /// (They may appear four times if the third and fourth are separated by a smaller value, such as
    /// XXXIX.)
    /// "D", "L", and "V" can never be repeated.
    /// "I" can be subtracted from "V" and "X" only.
    /// "X" can be subtracted from "L" and "C" only.
    /// "C" can be subtracted from "D" and "M" only.
    /// "V", "L", and "D" can never be subtracted.
    /// Only one small-value symbol may be subtracted from any large-value symbol.
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseRomanNumeralError);
        }
        // contains invalid character
        if !s
            .chars()
            .all(|c| ROMAN_VALUES.keys().collect::<Vec<&char>>().contains(&&c))
        {
            return Err(ParseRomanNumeralError);
        }

        // todo: actually implement...
        unimplemented!()

        //Ok(Roman {
        //    repr: s.to_string(),
        //    value,
        //})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_invalid_strings_raise_error() {
        let result = "".parse::<Roman>();
        match result {
            Err(e) => assert_eq!(e.to_string(), "invalid string for roman numeral"),
            _ => panic!("this should be an error"),
        }
    }

    #[test]
    fn test_roman_m_ok() {
        let result = "M".parse::<Roman>();
        match result {
            Ok(r) => assert_eq!(r.value, 1000),
            _ => panic!("this should be Ok"),
        }
    }
    #[test]
    fn test_roman_mm_ok() {
        let result = "MM".parse::<Roman>();
        match result {
            Ok(r) => assert_eq!(r.value, 2000),
            _ => panic!("this should be Ok"),
        }
    }
    #[test]
    fn test_roman_mmm_ok() {
        let result = "MMM".parse::<Roman>();
        match result {
            Ok(r) => assert_eq!(r.value, 3000),
            _ => panic!("this should be Ok"),
        }
    }
    #[test]
    fn test_roman_mmmm_err() {
        let result = "MMMM".parse::<Roman>();
        match result {
            Err(e) => assert_eq!(e, ParseRomanNumeralError),
            _ => panic!("this should not be Ok"),
        }
    }

    #[test]
    fn test_roman_mmmcm_ok() {
        let result = "MMMCM".parse::<Roman>();
        match result {
            Ok(r) => assert_eq!(r.value, 3900),
            _ => panic!("this should be Ok"),
        }
    }
    #[test]
    fn test_roman_mcm_ok() {
        let result = "MCM".parse::<Roman>();
        match result {
            Ok(r) => assert_eq!(r.value, 1900),
            _ => panic!("this should be Ok"),
        }
    }
    #[test]
    fn test_roman_mmcm_ok() {
        let result = "MMCM".parse::<Roman>();
        match result {
            Ok(r) => assert_eq!(r.value, 2900),
            _ => panic!("this should be Ok"),
        }
    }
    #[test]
    fn test_roman_mmmccm_err() {
        let result = "MMMCCM".parse::<Roman>();
        match result {
            Err(e) => assert_eq!(e, ParseRomanNumeralError),
            _ => panic!("this should not be Ok"),
        }
    }
    #[test]
    fn test_roman_cm_ok() {
        let result = "CM".parse::<Roman>();
        match result {
            Ok(r) => assert_eq!(r.value, 900),
            _ => panic!("this should be Ok"),
        }
    }
    #[test]
    fn test_roman_ccc_ok() {
        let result = "CCC".parse::<Roman>();
        match result {
            Ok(r) => assert_eq!(r.value, 300),
            _ => panic!("this should be Ok"),
        }
    }
    #[test]
    fn test_roman_cccc_err() {
        let result = "CCCC".parse::<Roman>();
        match result {
            Err(e) => assert_eq!(e, ParseRomanNumeralError),
            _ => panic!("this should not be Ok"),
        }
    }
    #[test]
    fn test_roman_cmcd_err() {
        let result = "CMCD".parse::<Roman>();
        match result {
            Err(e) => assert_eq!(e, ParseRomanNumeralError),
            _ => panic!("this should not be Ok"),
        }
    }

    #[test]
    fn test_roman_examples_ok() {
        let result = "M".parse::<Roman>();
        match result {
            Ok(r) => assert_eq!(r.value, 1000),
            _ => panic!("this should be Ok"),
        }
        let result = "CM".parse::<Roman>();
        match result {
            Ok(r) => assert_eq!(r.value, 900),
            _ => panic!("this should be Ok"),
        }
        let result = "III".parse::<Roman>();
        match result {
            Ok(r) => assert_eq!(r.value, 3),
            _ => panic!("this should be Ok"),
        }
        let result = "MCMIII".parse::<Roman>();
        match result {
            Ok(r) => assert_eq!(r.value, 1903),
            _ => panic!("this should be Ok"),
        }
    }
    #[test]
    fn test_roman_misc_ok() {
        let result = "MMMCM".parse::<Roman>();
        match result {
            Ok(r) => assert_eq!(r.value, 3900),
            _ => panic!("this should be Ok"),
        }
        let result = "MMCDLXXV".parse::<Roman>();
        match result {
            Ok(r) => assert_eq!(r.value, 2475),
            _ => panic!("this should be Ok"),
        }
        let result = "MDCCCLXXV".parse::<Roman>();
        match result {
            Ok(r) => assert_eq!(r.value, 1875),
            _ => panic!("this should be Ok"),
        }
    }
}
