use std::fmt;

/// Errors that can occur when working with betting odds.
///
/// This enum provides detailed error information for various invalid odds scenarios,
/// helping users understand what went wrong and how to fix it.
///
/// # Examples
///
/// ```
/// use odds_converter::{Odds, OddsError};
///
/// let invalid_odds = Odds::new_american(0);
/// match invalid_odds.validate() {
///     Err(OddsError::InvalidAmericanOdds(msg)) => {
///         println!("Invalid American odds: {}", msg);
///     }
///     _ => {}
/// }
/// ```
#[derive(Debug, PartialEq)]
pub enum OddsError {
    /// American odds format is invalid.
    ///
    /// This occurs when American odds are zero, -100 (which would imply infinite probability),
    /// or outside reasonable ranges.
    InvalidAmericanOdds(String),

    /// Decimal odds format is invalid.
    ///
    /// This occurs when decimal odds are less than 1.0, infinite, NaN, or outside
    /// reasonable ranges.
    InvalidDecimalOdds(String),

    /// Fractional odds format is invalid.
    ///
    /// This occurs when fractional odds have invalid values or are outside
    /// reasonable ranges.
    InvalidFractionalOdds(String),

    /// Failed to parse odds from a string.
    ///
    /// This occurs when a string cannot be interpreted as any valid odds format,
    /// or when the string format is malformed.
    ParseError(String),

    /// Odds value is outside the acceptable range.
    ///
    /// This occurs when odds values are technically valid but unreasonably large
    /// or small for practical betting scenarios.
    ValueOutOfRange(String),

    /// Fractional odds have a zero denominator.
    ///
    /// This is a special case of invalid fractional odds where division by zero
    /// would occur.
    ZeroDenominator,

    /// A negative value was provided where only positive values are allowed.
    ///
    /// This occurs in contexts where negative values don't make mathematical sense.
    NegativeValue(String),

    /// A non-finite number (infinity or NaN) was provided.
    ///
    /// This occurs when decimal odds are infinite or not-a-number, which cannot
    /// represent valid betting odds.
    InfiniteOrNaN,
}

impl fmt::Display for OddsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OddsError::InvalidAmericanOdds(msg) => write!(f, "Invalid American odds: {}", msg),
            OddsError::InvalidDecimalOdds(msg) => write!(f, "Invalid decimal odds: {}", msg),
            OddsError::InvalidFractionalOdds(msg) => write!(f, "Invalid fractional odds: {}", msg),
            OddsError::ParseError(msg) => write!(f, "Failed to parse odds string: {}", msg),
            OddsError::ValueOutOfRange(msg) => write!(f, "Value out of range: {}", msg),
            OddsError::ZeroDenominator => write!(f, "Denominator cannot be zero"),
            OddsError::NegativeValue(msg) => write!(f, "Negative value not allowed: {}", msg),
            OddsError::InfiniteOrNaN => write!(f, "Value must be finite and not NaN"),
        }
    }
}

impl std::error::Error for OddsError {}
