//! Display and parsing functionality for odds.
//!
//! This module implements string formatting and parsing for odds, allowing easy
//! conversion between odds and their string representations.

use crate::{Odds, OddsError, OddsFormat};
use std::fmt;
use std::str::FromStr;

impl fmt::Display for Odds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.format {
            OddsFormat::American(value) => {
                if *value > 0 {
                    write!(f, "+{}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            OddsFormat::Decimal(value) => write!(f, "{:.2}", value),
            OddsFormat::Fractional(num, den) => write!(f, "{}/{}", num, den),
        }
    }
}

impl FromStr for Odds {
    type Err = OddsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() {
            return Err(OddsError::ParseError("Empty string".to_string()));
        }

        // Try American format first (starts with + or - or is just a number)
        if s.starts_with('+') || s.starts_with('-') || s.chars().all(|c| c.is_ascii_digit()) {
            if let Ok(value) = s.parse::<i32>() {
                let odds = Odds::new_american(value);
                odds.validate()?;
                return Ok(odds);
            } else if s.starts_with('+') || s.starts_with('-') {
                return Err(OddsError::ParseError(format!(
                    "Invalid American odds format: '{}'",
                    s
                )));
            }
        }

        // Try fractional format (contains /)
        if s.contains('/') {
            let parts: Vec<&str> = s.split('/').collect();
            if parts.len() != 2 {
                return Err(OddsError::ParseError(format!(
                    "Invalid fractional format, expected 'num/den': '{}'",
                    s
                )));
            }

            let num_str = parts[0].trim();
            let den_str = parts[1].trim();

            if num_str.is_empty() || den_str.is_empty() {
                return Err(OddsError::ParseError(
                    "Empty numerator or denominator in fraction".to_string(),
                ));
            }

            match (num_str.parse::<u32>(), den_str.parse::<u32>()) {
                (Ok(num), Ok(den)) => {
                    let odds = Odds::new_fractional(num, den);
                    odds.validate()?;
                    return Ok(odds);
                }
                (Err(_), _) => {
                    return Err(OddsError::ParseError(format!(
                        "Invalid numerator: '{}'",
                        num_str
                    )))
                }
                (_, Err(_)) => {
                    return Err(OddsError::ParseError(format!(
                        "Invalid denominator: '{}'",
                        den_str
                    )))
                }
            }
        }

        // Try decimal format
        if let Ok(value) = s.parse::<f64>() {
            let odds = Odds::new_decimal(value);
            odds.validate()?;
            return Ok(odds);
        }

        Err(OddsError::ParseError(format!(
            "Unable to parse '{}' as any odds format",
            s
        )))
    }
}
