use odds_converter::{Odds, OddsError, OddsFormat};

#[test]
fn test_public_api_completeness() {
    // Test all public constructors
    let american = Odds::new_american(150);
    let decimal = Odds::new_decimal(2.5);
    let fractional = Odds::new_fractional(3, 2);

    // Test format access
    assert!(matches!(american.format(), OddsFormat::American(150)));
    assert!(matches!(decimal.format(), OddsFormat::Decimal(x) if (*x - 2.5).abs() < f64::EPSILON));
    assert!(matches!(fractional.format(), OddsFormat::Fractional(3, 2)));

    // Test all conversion methods
    assert!(american.to_decimal().is_ok());
    assert!(american.to_fractional().is_ok());
    assert!(american.implied_probability().is_ok());

    assert!(decimal.to_american().is_ok());
    assert!(decimal.to_fractional().is_ok());
    assert!(decimal.implied_probability().is_ok());

    assert!(fractional.to_american().is_ok());
    assert!(fractional.to_decimal().is_ok());
    assert!(fractional.implied_probability().is_ok());

    // Test validation
    assert!(american.validate().is_ok());
    assert!(decimal.validate().is_ok());
    assert!(fractional.validate().is_ok());
}

#[test]
fn test_error_handling_integration() {
    // Test that all error types can be created and displayed
    let zero_american = Odds::new_american(0);
    let error = zero_american.validate().unwrap_err();
    assert!(format!("{}", error).contains("cannot be zero"));

    let small_decimal = Odds::new_decimal(0.5);
    let error = small_decimal.validate().unwrap_err();
    assert!(format!("{}", error).contains("must be >= 1.0"));

    let zero_den_fractional = Odds::new_fractional(1, 0);
    let error = zero_den_fractional.validate().unwrap_err();
    assert_eq!(error, OddsError::ZeroDenominator);

    // Test parsing errors
    let parse_error: Result<Odds, _> = "invalid".parse();
    assert!(parse_error.is_err());
    assert!(format!("{}", parse_error.unwrap_err()).contains("Unable to parse"));
}

#[test]
fn test_display_and_parsing_integration() {
    let test_cases = vec![
        Odds::new_american(150),
        Odds::new_american(-200),
        Odds::new_decimal(2.5),
        Odds::new_fractional(3, 2),
    ];

    for odds in test_cases {
        if odds.validate().is_ok() {
            let displayed = format!("{}", odds);
            let parsed: Result<Odds, _> = displayed.parse();

            assert!(
                parsed.is_ok(),
                "Failed to parse displayed odds: {}",
                displayed
            );

            let parsed_odds = parsed.unwrap();

            // Verify they represent the same odds by comparing decimal values
            let original_decimal = odds.to_decimal().unwrap();
            let parsed_decimal = parsed_odds.to_decimal().unwrap();

            assert!(
                (original_decimal - parsed_decimal).abs() < 0.01,
                "Round-trip mismatch: {} -> {} -> {}",
                original_decimal,
                displayed,
                parsed_decimal
            );
        }
    }
}

#[test]
fn test_real_world_scenarios() {
    // Scenario 1: Converting Vegas odds to European format
    let vegas_odds = Odds::new_american(-110); // Standard sports betting line
    assert!(vegas_odds.validate().is_ok());

    let european_decimal = vegas_odds.to_decimal().unwrap();
    assert!((european_decimal - 1.909).abs() < 0.01); // Approximately 1.91

    let probability = vegas_odds.implied_probability().unwrap();
    assert!((probability - 0.524).abs() < 0.01); // About 52.4%

    // Scenario 2: Converting UK fractional odds
    let uk_odds = Odds::new_fractional(9, 4); // 9/4 odds
    assert!(uk_odds.validate().is_ok());

    let american = uk_odds.to_american().unwrap();
    assert_eq!(american, 225); // Should be +225

    let decimal = uk_odds.to_decimal().unwrap();
    assert!((decimal - 3.25).abs() < 0.01); // 9/4 + 1 = 3.25

    // Scenario 3: Parsing from user input
    let user_inputs = vec!["+150", "-200", "2.50", "3/2"];

    for input in user_inputs {
        let parsed: Result<Odds, _> = input.parse();
        assert!(parsed.is_ok(), "Failed to parse user input: {}", input);

        let odds = parsed.unwrap();
        assert!(odds.validate().is_ok());
        assert!(odds.implied_probability().is_ok());
    }

    // Scenario 4: Error handling with invalid user input
    let invalid_inputs = vec!["", "abc", "1/0", "+", "0.5", "1000000"];

    for input in invalid_inputs {
        let parsed: Result<Odds, _> = input.parse();
        if let Ok(odds) = parsed {
            // If parsing succeeded, validation should catch invalid values
            assert!(
                odds.validate().is_err(),
                "Should have failed validation: {}",
                input
            );
        }
    }
}

#[test]
fn test_mathematical_properties() {
    // Test that implied probabilities are mathematically correct
    let test_odds = vec![
        Odds::new_decimal(1.5), // 66.67% probability
        Odds::new_decimal(2.0), // 50% probability
        Odds::new_decimal(3.0), // 33.33% probability
        Odds::new_decimal(4.0), // 25% probability
    ];

    for odds in test_odds {
        let probability = odds.implied_probability().unwrap();
        let expected_probability = 1.0 / odds.to_decimal().unwrap();

        assert!(
            (probability - expected_probability).abs() < 1e-10,
            "Probability calculation error for odds {}",
            odds.to_decimal().unwrap()
        );

        assert!(
            probability > 0.0 && probability <= 1.0,
            "Probability out of range: {}",
            probability
        );
    }
}

#[test]
fn test_edge_cases_integration() {
    // Test minimum valid decimal odds
    let min_odds = Odds::new_decimal(1.0);
    assert!(min_odds.validate().is_ok());
    assert_eq!(min_odds.implied_probability().unwrap(), 1.0);

    // Test very close to minimum
    let close_to_min = Odds::new_decimal(1.001);
    assert!(close_to_min.validate().is_ok());
    let american = close_to_min.to_american().unwrap();
    assert!(american < 0); // Should be very negative odds

    // Test high probability scenarios
    let high_prob_american = Odds::new_american(-2000);
    assert!(high_prob_american.validate().is_ok());
    let probability = high_prob_american.implied_probability().unwrap();
    assert!(probability > 0.95); // Very high probability

    // Test low probability scenarios
    let low_prob_american = Odds::new_american(2000);
    assert!(low_prob_american.validate().is_ok());
    let probability = low_prob_american.implied_probability().unwrap();
    assert!(probability < 0.05); // Very low probability
}

#[test]
fn test_american_odds_edge_case_normalization() {
    // Test that positive odds between 1-99 are converted to negative equivalents
    
    // Test case: +50 should become -200
    let odds_50 = Odds::new_american(50);
    if let OddsFormat::American(value) = odds_50.format() {
        assert_eq!(*value, -200, "50 should be normalized to -200");
    } else {
        panic!("Expected American format");
    }
    
    // Test case: +25 should become -400
    let odds_25 = Odds::new_american(25);
    if let OddsFormat::American(value) = odds_25.format() {
        assert_eq!(*value, -400, "25 should be normalized to -400");
    } else {
        panic!("Expected American format");
    }
    
    // Test case: +10 should become -1000
    let odds_10 = Odds::new_american(10);
    if let OddsFormat::American(value) = odds_10.format() {
        assert_eq!(*value, -1000, "10 should be normalized to -1000");
    } else {
        panic!("Expected American format");
    }
    
    // Test edge cases: +1 and +99
    let odds_1 = Odds::new_american(1);
    if let OddsFormat::American(value) = odds_1.format() {
        assert_eq!(*value, -10000, "1 should be normalized to -10000");
    }
    
    let odds_99 = Odds::new_american(99);
    if let OddsFormat::American(value) = odds_99.format() {
        assert_eq!(*value, -101, "99 should be normalized to -101");
    }
    
    // Test that values >= 100 are NOT normalized
    let odds_100 = Odds::new_american(100);
    if let OddsFormat::American(value) = odds_100.format() {
        assert_eq!(*value, 100, "100 should remain unchanged");
    }
    
    let odds_150 = Odds::new_american(150);
    if let OddsFormat::American(value) = odds_150.format() {
        assert_eq!(*value, 150, "150 should remain unchanged");
    }
    
    // Test that negative values between -1 and -99 ARE normalized to positive
    let odds_neg50 = Odds::new_american(-50);
    if let OddsFormat::American(value) = odds_neg50.format() {
        assert_eq!(*value, 200, "-50 should be normalized to +200");
    }
    
    let odds_neg25 = Odds::new_american(-25);
    if let OddsFormat::American(value) = odds_neg25.format() {
        assert_eq!(*value, 400, "-25 should be normalized to +400");
    }
    
    let odds_neg10 = Odds::new_american(-10);
    if let OddsFormat::American(value) = odds_neg10.format() {
        assert_eq!(*value, 1000, "-10 should be normalized to +1000");
    }
    
    // Test edge cases: -1 and -99
    let odds_neg1 = Odds::new_american(-1);
    if let OddsFormat::American(value) = odds_neg1.format() {
        assert_eq!(*value, 10000, "-1 should be normalized to +10000");
    }
    
    let odds_neg99 = Odds::new_american(-99);
    if let OddsFormat::American(value) = odds_neg99.format() {
        assert_eq!(*value, 101, "-99 should be normalized to +101");
    }
    
    // Test that values <= -100 are NOT normalized
    let odds_neg100 = Odds::new_american(-100);
    if let OddsFormat::American(value) = odds_neg100.format() {
        assert_eq!(*value, -100, "-100 should remain unchanged");
    }
    
    let odds_neg150 = Odds::new_american(-150);
    if let OddsFormat::American(value) = odds_neg150.format() {
        assert_eq!(*value, -150, "-150 should remain unchanged");
    }
}

#[test]
fn test_negative_american_odds_edge_case_normalization() {
    // Test that negative odds between -1 and -99 are converted to positive equivalents
    
    // Test case: -50 should become +200
    let odds_neg50 = Odds::new_american(-50);
    if let OddsFormat::American(value) = odds_neg50.format() {
        assert_eq!(*value, 200, "-50 should be normalized to +200");
    }
    
    // Test case: -25 should become +400
    let odds_neg25 = Odds::new_american(-25);
    if let OddsFormat::American(value) = odds_neg25.format() {
        assert_eq!(*value, 400, "-25 should be normalized to +400");
    }
    
    // Test case: -10 should become +1000
    let odds_neg10 = Odds::new_american(-10);
    if let OddsFormat::American(value) = odds_neg10.format() {
        assert_eq!(*value, 1000, "-10 should be normalized to +1000");
    }
    
    // Test mathematical equivalence for negative normalization
    let test_cases = vec![
        (-1, 10000),
        (-5, 2000),
        (-10, 1000),
        (-20, 500),
        (-25, 400),
        (-50, 200),
        (-99, 101),
    ];
    
    for (input, expected) in test_cases {
        let odds = Odds::new_american(input);
        if let OddsFormat::American(value) = odds.format() {
            assert_eq!(*value, expected, "{} should normalize to +{}", input, expected);
        }
        
        // Verify the normalized odds represent the same probability
        let decimal_original = Odds::new_american(input).to_decimal().unwrap();
        let decimal_normalized = odds.to_decimal().unwrap();
        assert!((decimal_original - decimal_normalized).abs() < 0.001,
            "Decimal conversion should be identical: {} vs {}", decimal_original, decimal_normalized);
    }
}

#[test] 
fn test_conversion_to_american_never_returns_positive_0_100() {
    // Test that decimal to American conversion avoids positive 0-100 range
    
    // These decimal odds would normally produce positive values < 100
    let test_cases = vec![
        (1.01, "Very close to even money"),
        (1.1, "Slight favorite"),
        (1.25, "Moderate favorite"),
        (1.5, "Strong favorite"),
        (1.75, "Heavy favorite"),
        (1.99, "Just under even money"),
    ];
    
    for (decimal, description) in test_cases {
        let odds = Odds::new_decimal(decimal);
        let american = odds.to_american().unwrap();
        
        // Should never return positive values between 1-99
        if american > 0 {
            assert!(american >= 100, 
                "American odds conversion for {} (decimal {}) returned invalid positive value: {}", 
                description, decimal, american);
        }
        
        // Verify the conversion is mathematically consistent
        let back_to_decimal = Odds::new_american(american).to_decimal().unwrap();
        assert!(
            (decimal - back_to_decimal).abs() < 0.01,
            "Round-trip conversion failed for {}: {} -> {} -> {}",
            description, decimal, american, back_to_decimal
        );
    }
    
    // Test fractional odds that would produce problematic American odds
    let fractional_test_cases = vec![
        (1, 100, "1/100 odds"),
        (1, 50, "1/50 odds"), 
        (1, 25, "1/25 odds"),
        (1, 10, "1/10 odds"),
        (1, 4, "1/4 odds"),
        (1, 2, "1/2 odds"),
    ];
    
    for (num, den, description) in fractional_test_cases {
        let odds = Odds::new_fractional(num, den);
        let american = odds.to_american().unwrap();
        
        // Should never return positive values between 1-99
        if american > 0 {
            assert!(american >= 100,
                "Fractional to American conversion for {} returned invalid positive value: {}",
                description, american);
        }
    }
}

#[test]
fn test_normalized_odds_mathematical_consistency() {
    // Test that normalized odds maintain mathematical consistency
    
    let positive_test_values = vec![1, 5, 10, 25, 50, 75, 99];
    let negative_test_values = vec![-1, -5, -10, -25, -50, -75, -99];
    
    // Test positive odds normalization (1-99 -> negative)
    for original in positive_test_values {
        let odds = Odds::new_american(original);
        
        // Get the normalized American value
        let american = odds.to_american().unwrap();
        assert!(american < 0, "Normalized odds should be negative for positive input {}", original);
        
        // Convert to decimal and back to verify consistency
        let decimal = odds.to_decimal().unwrap();
        let back_to_american = Odds::new_decimal(decimal).to_american().unwrap();
        
        assert_eq!(american, back_to_american,
            "Round-trip conversion inconsistent for {}: {} -> {} -> {}",
            original, american, decimal, back_to_american);
        
        // Verify implied probability is valid
        let probability = odds.implied_probability().unwrap();
        assert!(probability > 0.0 && probability <= 1.0,
            "Invalid probability {} for normalized odds from input {}",
            probability, original);
        
        // Verify that the probability is greater than 50% (since we're dealing with favorites)
        assert!(probability > 0.5,
            "Normalized odds should represent favorites (probability > 50%), got {} for input {}",
            probability, original);
    }
    
    // Test negative odds normalization (-1 to -99 -> positive)
    for original in negative_test_values {
        let odds = Odds::new_american(original);
        
        // Get the normalized American value
        let american = odds.to_american().unwrap();
        assert!(american > 0, "Normalized odds should be positive for negative input {}", original);
        assert!(american >= 100, "Normalized odds should be >= 100 for negative input {}", original);
        
        // Convert to decimal and back to verify consistency
        let decimal = odds.to_decimal().unwrap();
        let back_to_american = Odds::new_decimal(decimal).to_american().unwrap();
        
        assert_eq!(american, back_to_american,
            "Round-trip conversion inconsistent for {}: {} -> {} -> {}",
            original, american, decimal, back_to_american);
        
        // Verify implied probability is valid
        let probability = odds.implied_probability().unwrap();
        assert!(probability > 0.0 && probability <= 1.0,
            "Invalid probability {} for normalized odds from input {}",
            probability, original);
        
        // Verify that the probability is less than 50% (since we're dealing with underdogs)
        assert!(probability < 0.5,
            "Normalized odds should represent underdogs (probability < 50%), got {} for input {}",
            probability, original);
    }
}
