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
