use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

lazy_static! {
    pub static ref ROMAN_VALUES: HashMap<char, i32> = {
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
    pub(crate) repr: String,
    pub(crate) value: i32,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.repr, self.value)
    }
}

// from integer to Roman numeral
impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        if num >= 4000 {
            panic!(
                "{} cannot be converted. Hint: Only uint values <= 4000 can be represented. ",
                num
            )
        }
        let mut result = "".to_string();
        let thousands = num / 1000;
        let hundreds = (num - thousands * 1000) / 100;
        let tens = (num - thousands * 1000 - hundreds * 100) / 10;
        let units = num - thousands * 1000 - hundreds * 100 - tens * 10;
        //todo: refactor

        if let 0..=3 = thousands {
            for _ in 0..thousands {
                result.push('M')
            }
        }

        match hundreds {
            0..=3 => {
                for _ in 0..hundreds {
                    result.push('C')
                }
            }
            4 => result.push_str("CD"),
            5 => result.push('D'),
            6 => result.push_str("DC"),
            7 => result.push_str("DCC"),
            8 => result.push_str("DCCC"),
            9 => result.push_str("CM"),
            _ => {}
        }
        match tens {
            0..=3 => {
                for _ in 0..tens {
                    result.push('X')
                }
            }
            4 => result.push_str("XL"),
            5 => result.push('L'),
            6 => result.push_str("LX"),
            7 => result.push_str("LXX"),
            8 => result.push_str("LXXX"),
            9 => result.push_str("XC"),
            _ => {}
        }
        match units {
            0..=3 => {
                for _ in 0..units {
                    result.push('I')
                }
            }
            4 => result.push_str("IV"),
            5 => result.push('V'),
            6 => result.push_str("VI"),
            7 => result.push_str("VII"),
            8 => result.push_str("VIII"),
            9 => result.push_str("IX"),
            _ => {}
        }

        Roman {
            repr: result,
            value: num as i32,
        }
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
        if !s.chars().all(|c| ROMAN_VALUES.keys().any(|x| x == &c)) {
            return Err(ParseRomanNumeralError);
        }

        // a bit hacky...
        // Perform math on the input characters (e.g. add 10 for X
        // (or subtract 10 if followed by numeral for larger number, respectively) etc.
        // Check if result is within representable bounds (0<=result<4000).
        // If so, convert the result of the calculation to roman numerals
        // via Roman::from<u32> implemented above
        // Check if the original representation matches the newly calculated representation.
        // If they match, the input was valid, otherwise return ParseRomanNumeralError

        //convert single numerals to their values
        let values: Vec<i32> = s.chars().map(|c| ROMAN_VALUES[&c]).collect();
        let mut sum = 0;
        for (i, v) in values.iter().enumerate() {
            if let Some(next_value) = values.get(i + 1) {
                if v < next_value {
                    sum -= v
                } else {
                    sum += v
                }
            } else {
                sum += v //last
            }
        }
        if sum <= 0 || sum >= 4000 {
            return Err(ParseRomanNumeralError);
        }

        let valid_numeral_for_result = Roman::from(sum as u32).repr;
        if valid_numeral_for_result != s {
            return Err(ParseRomanNumeralError);
        }

        Ok(Roman {
            repr: s.to_string(),
            value: sum,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /* uint to Roman */
    #[test]
    fn test_int_to_roman_1000() {
        assert_eq!(Roman::from(1000).repr, "M")
    }
    #[test]
    fn test_int_to_roman_900() {
        assert_eq!(Roman::from(900).repr, "CM")
    }
    #[test]
    fn test_int_to_roman_3() {
        assert_eq!(Roman::from(3).repr, "III")
    }
    #[test]
    fn test_int_to_roman_1903() {
        assert_eq!(Roman::from(1903).repr, "MCMIII")
    }
    #[test]
    #[should_panic]
    fn test_int_to_roman_4000() {
        let _that_wont_work = Roman::from(4000);
    }
    #[test]
    fn test_int_to_roman_3999() {
        assert_eq!(Roman::from(3999).repr, "MMMCMXCIX")
    }
    #[test]
    fn test_int_to_roman_42() {
        assert_eq!(Roman::from(42).repr, "XLII")
    }
    /* Roman to int */
    #[test]
    fn test_roman_invalid_strings_raise_error() {
        let result = "".parse::<Roman>();
        match result {
            Err(e) => assert_eq!(e.to_string(), "invalid string for roman numeral"),
            _ => panic!("this should be an error"),
        }
    }
    #[test]
    fn test_roman_invalid_strings_raise_error_y() {
        let result = "Y".parse::<Roman>();
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
