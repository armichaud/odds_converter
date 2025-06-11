//! A Rust library for converting between different betting odds formats.
//!
//! This crate provides functionality to convert between American, Decimal, and Fractional
//! betting odds formats, along with implied probability calculations and robust error handling.
//!
//! # Examples
//!
//! ```
//! use odds_converter::Odds;
//!
//! // Create odds in different formats
//! let american = Odds::new_american(150);
//! let decimal = Odds::new_decimal(2.5);
//! let fractional = Odds::new_fractional(3, 2);
//!
//! // Convert between formats
//! let decimal_from_american = american.to_decimal().unwrap();
//! let american_from_decimal = decimal.to_american().unwrap();
//!
//! // Calculate implied probability
//! let probability = decimal.implied_probability().unwrap();
//!
//! // Parse from strings
//! let odds: Odds = "+150".parse().unwrap();
//! ```

mod conversions;
mod display;
mod error;
mod types;
mod validation;

// Re-export public types
pub use error::OddsError;
pub use types::{Odds, OddsFormat};

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_odds_creation() {
        let american = Odds::new_american(150);
        let decimal = Odds::new_decimal(2.5);
        let fractional = Odds::new_fractional(3, 2);

        assert_eq!(american.format(), &OddsFormat::American(150));
        assert_eq!(decimal.format(), &OddsFormat::Decimal(2.5));
        assert_eq!(fractional.format(), &OddsFormat::Fractional(3, 2));
    }

    #[test]
    fn test_american_to_decimal_conversion() {
        let positive_american = Odds::new_american(150);
        assert_eq!(positive_american.to_decimal().unwrap(), 2.5);

        let negative_american = Odds::new_american(-200);
        assert_eq!(negative_american.to_decimal().unwrap(), 1.5);
    }

    #[test]
    fn test_decimal_to_american_conversion() {
        let decimal = Odds::new_decimal(2.5);
        assert_eq!(decimal.to_american().unwrap(), 150);

        let decimal2 = Odds::new_decimal(1.5);
        assert_eq!(decimal2.to_american().unwrap(), -200);
    }

    #[test]
    fn test_fractional_to_decimal_conversion() {
        let fractional = Odds::new_fractional(3, 2);
        assert_eq!(fractional.to_decimal().unwrap(), 2.5);

        let fractional2 = Odds::new_fractional(1, 2);
        assert_eq!(fractional2.to_decimal().unwrap(), 1.5);
    }

    #[test]
    fn test_decimal_to_fractional_conversion() {
        let decimal = Odds::new_decimal(2.5);
        let (num, den) = decimal.to_fractional().unwrap();
        assert_eq!(num as f64 / den as f64 + 1.0, 2.5);
    }

    #[test]
    fn test_implied_probability() {
        let decimal = Odds::new_decimal(2.0);
        assert_eq!(decimal.implied_probability().unwrap(), 0.5);

        let american = Odds::new_american(100);
        assert!((american.implied_probability().unwrap() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_validation() {
        let invalid_american = Odds::new_american(0);
        assert!(invalid_american.validate().is_err());

        let invalid_decimal = Odds::new_decimal(0.5);
        assert!(invalid_decimal.validate().is_err());

        let invalid_fractional = Odds::new_fractional(1, 0);
        assert!(invalid_fractional.validate().is_err());

        let valid_american = Odds::new_american(150);
        assert!(valid_american.validate().is_ok());
    }

    #[test]
    fn test_display_formatting() {
        let american_pos = Odds::new_american(150);
        assert_eq!(format!("{}", american_pos), "+150");

        let american_neg = Odds::new_american(-200);
        assert_eq!(format!("{}", american_neg), "-200");

        let decimal = Odds::new_decimal(2.5);
        assert_eq!(format!("{}", decimal), "2.50");

        let fractional = Odds::new_fractional(3, 2);
        assert_eq!(format!("{}", fractional), "3/2");
    }

    #[test]
    fn test_string_parsing() {
        let american_pos: Odds = "+150".parse().unwrap();
        assert_eq!(american_pos.format(), &OddsFormat::American(150));

        let american_neg: Odds = "-200".parse().unwrap();
        assert_eq!(american_neg.format(), &OddsFormat::American(-200));

        let decimal: Odds = "2.5".parse().unwrap();
        assert_eq!(decimal.format(), &OddsFormat::Decimal(2.5));

        let fractional: Odds = "3/2".parse().unwrap();
        assert_eq!(fractional.format(), &OddsFormat::Fractional(3, 2));

        let invalid: Result<Odds, _> = "invalid".parse();
        assert!(invalid.is_err());
    }

    #[test]
    fn test_round_trip_conversions() {
        // American -> Decimal -> American
        let original_american = Odds::new_american(150);
        let decimal = original_american.to_decimal().unwrap();
        let back_to_american = Odds::new_decimal(decimal).to_american().unwrap();
        assert_eq!(original_american.to_american().unwrap(), back_to_american);

        // Decimal -> Fractional -> Decimal (approximate)
        let original_decimal = Odds::new_decimal(2.5);
        let (num, den) = original_decimal.to_fractional().unwrap();
        let back_to_decimal = Odds::new_fractional(num, den).to_decimal().unwrap();
        assert!((original_decimal.to_decimal().unwrap() - back_to_decimal).abs() < 0.01);
    }

    #[test]
    fn test_edge_case_validations() {
        // Test -100 American odds (50% probability - even odds, should be valid)
        let minus_100_american = Odds::new_american(-100);
        assert!(minus_100_american.validate().is_ok());

        // Test very large values
        let large_american = Odds::new_american(150000);
        assert!(large_american.validate().is_err());

        // Test decimal odds exactly 1.0 (should be valid)
        let decimal_one = Odds::new_decimal(1.0);
        assert!(decimal_one.validate().is_ok());

        // Test very large decimal odds
        let large_decimal = Odds::new_decimal(1500.0);
        assert!(large_decimal.validate().is_err());

        // Test infinite and NaN values
        let infinite_decimal = Odds::new_decimal(f64::INFINITY);
        assert_eq!(infinite_decimal.validate(), Err(OddsError::InfiniteOrNaN));

        let nan_decimal = Odds::new_decimal(f64::NAN);
        assert_eq!(nan_decimal.validate(), Err(OddsError::InfiniteOrNaN));

        // Test large fractional odds
        let large_fractional = Odds::new_fractional(15000, 1);
        assert!(large_fractional.validate().is_err());
    }

    #[test]
    fn test_enhanced_parsing_errors() {
        // Test empty string
        let empty: Result<Odds, _> = "".parse();
        assert!(matches!(empty, Err(OddsError::ParseError(_))));

        // Test invalid American format
        let invalid_american: Result<Odds, _> = "+abc".parse();
        assert!(matches!(invalid_american, Err(OddsError::ParseError(_))));

        // Test invalid fractional format
        let invalid_fraction: Result<Odds, _> = "3/2/1".parse();
        assert!(matches!(invalid_fraction, Err(OddsError::ParseError(_))));

        // Test empty fraction parts
        let empty_numerator: Result<Odds, _> = "/2".parse();
        assert!(matches!(empty_numerator, Err(OddsError::ParseError(_))));

        let empty_denominator: Result<Odds, _> = "3/".parse();
        assert!(matches!(empty_denominator, Err(OddsError::ParseError(_))));

        // Test invalid numerator/denominator
        let invalid_num: Result<Odds, _> = "abc/2".parse();
        assert!(matches!(invalid_num, Err(OddsError::ParseError(_))));

        let invalid_den: Result<Odds, _> = "3/abc".parse();
        assert!(matches!(invalid_den, Err(OddsError::ParseError(_))));

        // Test zero denominator
        let zero_den: Result<Odds, _> = "3/0".parse();
        assert!(matches!(zero_den, Err(OddsError::ZeroDenominator)));
    }

    #[test]
    fn test_conversion_edge_cases() {
        // Test conversion of very small decimal odds
        let small_decimal = Odds::new_decimal(1.01);
        let american = small_decimal.to_american().unwrap();
        assert!(american < 0); // Should be negative odds

        // Test conversion of odds close to even (2.0)
        let even_decimal = Odds::new_decimal(2.0);
        assert_eq!(even_decimal.to_american().unwrap(), 100);

        // Test fractional to decimal edge case (0/1 -> 1.0)
        let zero_profit = Odds::new_fractional(0, 1);
        assert_eq!(zero_profit.to_decimal().unwrap(), 1.0);
    }

    #[test]
    fn test_detailed_error_messages() {
        let zero_american = Odds::new_american(0);
        let validation_result = zero_american.validate();
        if let Err(OddsError::InvalidAmericanOdds(msg)) = validation_result {
            assert!(msg.contains("cannot be zero"));
        } else {
            panic!("Expected InvalidAmericanOdds error");
        }

        let small_decimal = Odds::new_decimal(0.5);
        let validation_result = small_decimal.validate();
        if let Err(OddsError::InvalidDecimalOdds(msg)) = validation_result {
            assert!(msg.contains("0.5"));
        } else {
            panic!("Expected InvalidDecimalOdds error");
        }
    }

    // Property-based tests
    proptest! {
        #[test]
        fn prop_american_roundtrip(american in -10000i32..10000i32) {
            prop_assume!(american != 0 && american != -100 && american.abs() >= 100);

            let odds = Odds::new_american(american);
            if odds.validate().is_ok() {
                let decimal = odds.to_decimal().unwrap();
                let back_to_american = Odds::new_decimal(decimal).to_american().unwrap();

                // Allow small rounding errors
                prop_assert!((american - back_to_american).abs() <= 5,
                    "American {} -> Decimal {} -> American {} (diff: {})",
                    american, decimal, back_to_american, (american - back_to_american).abs());
            }
        }

        #[test]
        fn prop_decimal_roundtrip(decimal in 1.001f64..100.0f64) {
            prop_assume!(decimal.is_finite());

            let odds = Odds::new_decimal(decimal);
            if odds.validate().is_ok() {
                let american = odds.to_american().unwrap();
                let back_to_decimal = Odds::new_american(american).to_decimal().unwrap();

                // Allow small rounding errors (0.5%)
                let relative_error = ((decimal - back_to_decimal) / decimal).abs();
                prop_assert!(relative_error < 0.005,
                    "Decimal {} -> American {} -> Decimal {} (relative error: {})",
                    decimal, american, back_to_decimal, relative_error);
            }
        }

        #[test]
        fn prop_fractional_roundtrip(num in 1u32..1000u32, den in 1u32..1000u32) {
            let odds = Odds::new_fractional(num, den);
            if odds.validate().is_ok() {
                let decimal = odds.to_decimal().unwrap();
                let (back_num, back_den) = Odds::new_decimal(decimal).to_fractional().unwrap();

                // Check that the decimal representation is equivalent
                let original_decimal = (num as f64) / (den as f64) + 1.0;
                let roundtrip_decimal = (back_num as f64) / (back_den as f64) + 1.0;

                let relative_error = ((original_decimal - roundtrip_decimal) / original_decimal).abs();
                prop_assert!(relative_error < 0.01,
                    "Fractional {}/{} -> Decimal {} -> Fractional {}/{} (relative error: {})",
                    num, den, decimal, back_num, back_den, relative_error);
            }
        }

        #[test]
        fn prop_implied_probability_valid(decimal in 1.001f64..100.0f64) {
            prop_assume!(decimal.is_finite());

            let odds = Odds::new_decimal(decimal);
            if odds.validate().is_ok() {
                let prob = odds.implied_probability().unwrap();

                // Probability should be between 0 and 1
                prop_assert!(prob > 0.0 && prob <= 1.0,
                    "Invalid probability {} for decimal odds {}", prob, decimal);

                // Should equal 1/decimal
                let expected_prob = 1.0 / decimal;
                prop_assert!((prob - expected_prob).abs() < 1e-10,
                    "Probability mismatch: got {}, expected {}", prob, expected_prob);
            }
        }

        #[test]
        fn prop_string_parsing_roundtrip(american in -10000i32..10000i32) {
            prop_assume!(american != 0 && american != -100);

            let odds = Odds::new_american(american);
            if odds.validate().is_ok() {
                let formatted = format!("{}", odds);
                let parsed: Result<Odds, _> = formatted.parse();

                if let Ok(parsed_odds) = parsed {
                    prop_assert_eq!(odds.to_american().unwrap(), parsed_odds.to_american().unwrap());
                }
            }
        }

        #[test]
        fn prop_conversion_consistency(decimal in 1.001f64..10.0f64) {
            prop_assume!(decimal.is_finite());

            let decimal_odds = Odds::new_decimal(decimal);
            if decimal_odds.validate().is_ok() {
                let american = decimal_odds.to_american().unwrap();
                let (num, den) = decimal_odds.to_fractional().unwrap();

                // All three should have the same implied probability
                let decimal_prob = decimal_odds.implied_probability().unwrap();
                let american_prob = Odds::new_american(american).implied_probability().unwrap();
                let fractional_prob = Odds::new_fractional(num, den).implied_probability().unwrap();

                prop_assert!((decimal_prob - american_prob).abs() < 0.01,
                    "Probability mismatch between decimal and American: {} vs {}",
                    decimal_prob, american_prob);

                prop_assert!((decimal_prob - fractional_prob).abs() < 0.01,
                    "Probability mismatch between decimal and fractional: {} vs {}",
                    decimal_prob, fractional_prob);
            }
        }
    }

    #[test]
    fn test_boundary_values() {
        // Test exactly at boundaries
        let min_decimal = Odds::new_decimal(1.0);
        assert!(min_decimal.validate().is_ok());
        assert_eq!(min_decimal.implied_probability().unwrap(), 1.0);

        let large_decimal = Odds::new_decimal(999.99);
        assert!(large_decimal.validate().is_ok());

        let max_decimal = Odds::new_decimal(1000.0);
        assert!(max_decimal.validate().is_ok());

        let too_large_decimal = Odds::new_decimal(1000.01);
        assert!(too_large_decimal.validate().is_err());

        // Test American odds boundaries
        let max_american = Odds::new_american(99999);
        assert!(max_american.validate().is_ok());

        let min_american = Odds::new_american(-99999);
        assert!(min_american.validate().is_ok());

        let too_large_american = Odds::new_american(100001);
        assert!(too_large_american.validate().is_err());

        // Test fractional boundaries
        let max_fractional = Odds::new_fractional(9999, 1);
        assert!(max_fractional.validate().is_ok());

        let too_large_fractional = Odds::new_fractional(10001, 1);
        assert!(too_large_fractional.validate().is_err());
    }

    #[test]
    fn test_mathematical_correctness() {
        // Test specific known conversions
        let test_cases = vec![
            (100, 2.0),     // Even odds (1:1)
            (150, 2.5),     // 3:2 odds
            (-200, 1.5),    // 1:2 odds
            (300, 4.0),     // 3:1 odds
            (-150, 1.6667), // 2:3 odds (approximately)
        ];

        for (american, expected_decimal) in test_cases {
            let american_odds = Odds::new_american(american);
            let decimal = american_odds.to_decimal().unwrap();

            // Check decimal conversion (allow small rounding errors)
            assert!(
                (decimal - expected_decimal).abs() < 0.01,
                "American {} should convert to ~{}, got {}",
                american,
                expected_decimal,
                decimal
            );

            // Check fractional conversion produces equivalent odds
            let (num, den) = american_odds.to_fractional().unwrap();
            let fractional_decimal = (num as f64) / (den as f64) + 1.0;
            assert!(
                (fractional_decimal - expected_decimal).abs() < 0.1,
                "Fractional {}/{} should be equivalent to decimal {}",
                num,
                den,
                expected_decimal
            );
        }
    }
}
