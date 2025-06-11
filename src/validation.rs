//! Odds validation functionality.
//!
//! This module contains validation logic to ensure odds values are mathematically
//! valid and within reasonable ranges for practical betting scenarios.

use crate::{Odds, OddsError, OddsFormat};

impl Odds {
    /// Validates that the odds are mathematically correct and within reasonable ranges.
    ///
    /// This method checks that odds values make mathematical sense and are within
    /// practical limits for real-world betting scenarios. It validates:
    ///
    /// - American odds are not zero or -100 (infinite probability)
    /// - Decimal odds are >= 1.0 and finite
    /// - Fractional odds don't have zero denominators
    /// - All odds are within reasonable ranges
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the odds are valid, or an `Err(OddsError)` describing
    /// the specific validation failure.
    ///
    /// # Examples
    ///
    /// ```
    /// use odds_converter::{Odds, OddsError};
    ///
    /// let valid_odds = Odds::new_decimal(2.5);
    /// assert!(valid_odds.validate().is_ok());
    ///
    /// let invalid_odds = Odds::new_decimal(0.5);
    /// assert!(invalid_odds.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), OddsError> {
        match &self.format {
            OddsFormat::American(value) => {
                if *value == 0 {
                    Err(OddsError::InvalidAmericanOdds(
                        "American odds cannot be zero".to_string(),
                    ))
                } else if *value == -100 {
                    Err(OddsError::InvalidAmericanOdds(
                        "American odds cannot be -100 (would imply infinite probability)"
                            .to_string(),
                    ))
                } else if *value < -100000 || *value > 100000 {
                    Err(OddsError::ValueOutOfRange(format!(
                        "American odds out of reasonable range: {}",
                        value
                    )))
                } else {
                    Ok(())
                }
            }
            OddsFormat::Decimal(value) => {
                if !value.is_finite() {
                    Err(OddsError::InfiniteOrNaN)
                } else if *value < 1.0 {
                    Err(OddsError::InvalidDecimalOdds(format!(
                        "Decimal odds must be >= 1.0, got: {}",
                        value
                    )))
                } else if *value > 1000.0 {
                    Err(OddsError::ValueOutOfRange(format!(
                        "Decimal odds too large: {}",
                        value
                    )))
                } else {
                    Ok(())
                }
            }
            OddsFormat::Fractional(num, den) => {
                if *den == 0 {
                    Err(OddsError::ZeroDenominator)
                } else if *num > 10000 || *den > 10000 {
                    Err(OddsError::ValueOutOfRange(
                        "Fractional odds values too large".to_string(),
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }
}
