//! Odds conversion functionality.
//!
//! This module contains all the logic for converting between different odds formats
//! and calculating implied probabilities.

use crate::{Odds, OddsError, OddsFormat};
use num_integer::gcd;

/// Normalizes American odds to their standard representation.
/// 
/// This function handles edge cases in American odds notation:
/// - Positive odds between 1-99 are converted to equivalent negative odds
/// - Negative odds between -1 and -99 are converted to equivalent positive odds
///
/// Conversion formulas:
/// - For positive 1-99: -(100 * 100) / positive_odds
/// - For negative -1 to -99: (100 * 100) / abs(negative_odds)
/// 
/// Examples:
/// - +50 becomes -200, +25 becomes -400
/// - -50 becomes +200, -25 becomes +400
pub(crate) fn normalize_american_odds(odds: i32) -> i32 {
    if odds > 0 && odds < 100 {
        // Convert positive odds 1-99 to equivalent negative odds
        // Formula: -(100 * 100) / positive_odds
        -((100 * 100) / odds)
    } else if odds < 0 && odds > -100 {
        // Convert negative odds -1 to -99 to equivalent positive odds
        // Formula: (100 * 100) / abs(negative_odds)
        (100 * 100) / (-odds)
    } else {
        odds
    }
}

impl Odds {
    /// Converts odds to American format.
    ///
    /// American odds use positive numbers for underdogs (profit on $100 bet) and
    /// negative numbers for favorites (amount to bet to win $100).
    ///
    /// # Returns
    ///
    /// Returns `Ok(i32)` containing the American odds value, or an `Err(OddsError)`
    /// if the conversion fails due to invalid input values.
    ///
    /// # Examples
    ///
    /// ```
    /// use odds_converter::Odds;
    ///
    /// let decimal_odds = Odds::new_decimal(2.5);
    /// assert_eq!(decimal_odds.to_american().unwrap(), 150);
    ///
    /// let fractional_odds = Odds::new_fractional(1, 2);
    /// assert_eq!(fractional_odds.to_american().unwrap(), -200);
    /// ```
    pub fn to_american(&self) -> Result<i32, OddsError> {
        match &self.format {
            OddsFormat::American(value) => Ok(*value),
            OddsFormat::Decimal(decimal) => {
                if *decimal >= 2.0 {
                    let american = ((decimal - 1.0) * 100.0).round() as i32;
                    Ok(normalize_american_odds(american))
                } else if *decimal > 1.0 {
                    Ok((-100.0 / (decimal - 1.0)).round() as i32)
                } else {
                    Err(OddsError::InvalidDecimalOdds(format!(
                        "Decimal odds must be greater than 1.0, got: {}",
                        decimal
                    )))
                }
            }
            OddsFormat::Fractional(num, den) => {
                let decimal = (*num as f64) / (*den as f64) + 1.0;
                if decimal >= 2.0 {
                    let american = ((decimal - 1.0) * 100.0).round() as i32;
                    Ok(normalize_american_odds(american))
                } else {
                    Ok((-100.0 / (decimal - 1.0)).round() as i32)
                }
            }
        }
    }

    /// Converts odds to decimal format.
    ///
    /// Decimal odds represent the total return (including original stake) for a unit bet.
    /// This is the most straightforward format for mathematical calculations.
    ///
    /// # Returns
    ///
    /// Returns `Ok(f64)` containing the decimal odds value, or an `Err(OddsError)`
    /// if the conversion fails due to invalid input values.
    ///
    /// # Examples
    ///
    /// ```
    /// use odds_converter::Odds;
    ///
    /// let american_odds = Odds::new_american(150);
    /// assert_eq!(american_odds.to_decimal().unwrap(), 2.5);
    ///
    /// let fractional_odds = Odds::new_fractional(3, 2);
    /// assert_eq!(fractional_odds.to_decimal().unwrap(), 2.5);
    /// ```
    pub fn to_decimal(&self) -> Result<f64, OddsError> {
        match &self.format {
            OddsFormat::Decimal(value) => Ok(*value),
            OddsFormat::American(american) => {
                if *american > 0 {
                    Ok((*american as f64) / 100.0 + 1.0)
                } else if *american < 0 {
                    Ok(100.0 / (-*american as f64) + 1.0)
                } else {
                    Err(OddsError::InvalidAmericanOdds(
                        "American odds cannot be zero".to_string(),
                    ))
                }
            }
            OddsFormat::Fractional(num, den) => {
                if *den == 0 {
                    Err(OddsError::ZeroDenominator)
                } else {
                    Ok((*num as f64) / (*den as f64) + 1.0)
                }
            }
        }
    }

    /// Converts odds to fractional format.
    ///
    /// Fractional odds represent the ratio of profit to stake. The returned tuple
    /// contains (numerator, denominator) where numerator is the profit and
    /// denominator is the stake amount.
    ///
    /// # Returns
    ///
    /// Returns `Ok((u32, u32))` containing the fractional odds as (numerator, denominator),
    /// or an `Err(OddsError)` if the conversion fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use odds_converter::Odds;
    ///
    /// let decimal_odds = Odds::new_decimal(2.5);
    /// let (num, den) = decimal_odds.to_fractional().unwrap();
    /// // Should be equivalent to 3:2 odds (1.5 profit ratio + 1)
    /// assert_eq!((num as f64) / (den as f64) + 1.0, 2.5);
    /// ```
    pub fn to_fractional(&self) -> Result<(u32, u32), OddsError> {
        match &self.format {
            OddsFormat::Fractional(num, den) => Ok((*num, *den)),
            _ => {
                let decimal = self.to_decimal()?;
                let profit = decimal - 1.0;

                let denominator = 1000;
                let numerator = (profit * denominator as f64).round() as u32;

                let common_divisor = gcd(numerator, denominator);
                Ok((numerator / common_divisor, denominator / common_divisor))
            }
        }
    }

    /// Calculates the implied probability from the odds.
    ///
    /// Implied probability represents the likelihood of an event occurring according
    /// to the odds. It's calculated as 1 / decimal_odds and ranges from 0.0 to 1.0.
    ///
    /// # Returns
    ///
    /// Returns `Ok(f64)` containing the probability as a decimal between 0 and 1,
    /// or an `Err(OddsError)` if the calculation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use odds_converter::Odds;
    ///
    /// let even_odds = Odds::new_decimal(2.0);
    /// assert_eq!(even_odds.implied_probability().unwrap(), 0.5); // 50%
    ///
    /// let american_favorite = Odds::new_american(-200);
    /// let prob = american_favorite.implied_probability().unwrap();
    /// assert!((prob - 0.6667).abs() < 0.001); // ~66.67%
    /// ```
    pub fn implied_probability(&self) -> Result<f64, OddsError> {
        let decimal = self.to_decimal()?;
        Ok(1.0 / decimal)
    }
}
