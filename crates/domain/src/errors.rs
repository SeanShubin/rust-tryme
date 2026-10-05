//! Errors as values: `Result`, the `?` operator, and a custom error type.
//!
//! Rust has no exceptions. A function that can fail says so in its return type,
//! so the caller cannot ignore the failure by accident.

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseAgeError {
    NotANumber(String),
    Unreasonable(i64),
}

impl fmt::Display for ParseAgeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseAgeError::NotANumber(text) => write!(formatter, "not a number: {text}"),
            ParseAgeError::Unreasonable(value) => write!(formatter, "unreasonable age: {value}"),
        }
    }
}

impl std::error::Error for ParseAgeError {}

/// `?` returns early on failure, converting the error with `From` if needed.
/// Here the conversion is explicit via `map_err`, so the caller sees our error
/// type rather than the standard library's.
pub fn parse_age(text: &str) -> Result<u8, ParseAgeError> {
    let value: i64 = text
        .trim()
        .parse()
        .map_err(|_| ParseAgeError::NotANumber(text.to_string()))?;
    if (0..=130).contains(&value) {
        Ok(value as u8)
    } else {
        Err(ParseAgeError::Unreasonable(value))
    }
}

/// Collecting into a `Result<Vec<_>, _>` stops at the first failure, which is
/// usually what you want and is easy to miss.
pub fn parse_all(texts: &[&str]) -> Result<Vec<u8>, ParseAgeError> {
    texts.iter().map(|text| parse_age(text)).collect()
}

/// `Option` is the same idea for absence rather than failure.
pub fn oldest(ages: &[u8]) -> Option<u8> {
    ages.iter().copied().max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_reasonable_age() {
        assert_eq!(Ok(42), parse_age("42"));
        assert_eq!(Ok(0), parse_age(" 0 "));
    }

    #[test]
    fn rejects_bad_input_with_a_typed_error() {
        assert_eq!(
            Err(ParseAgeError::NotANumber("abc".to_string())),
            parse_age("abc")
        );
        assert_eq!(Err(ParseAgeError::Unreasonable(900)), parse_age("900"));
    }

    #[test]
    fn error_messages_come_from_display() {
        assert_eq!("not a number: abc", ParseAgeError::NotANumber("abc".to_string()).to_string());
        assert_eq!("unreasonable age: 900", ParseAgeError::Unreasonable(900).to_string());
    }

    #[test]
    fn collecting_results_stops_at_the_first_failure() {
        assert_eq!(Ok(vec![1, 2, 3]), parse_all(&["1", "2", "3"]));
        assert_eq!(
            Err(ParseAgeError::NotANumber("two".to_string())),
            parse_all(&["1", "two", "3"])
        );
    }

    #[test]
    fn option_models_absence() {
        assert_eq!(Some(9), oldest(&[3, 9, 1]));
        assert_eq!(None, oldest(&[]));
    }
}
