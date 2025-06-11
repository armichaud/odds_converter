# Odds Converter

A Rust library for converting between different betting odds formats with robust error handling and comprehensive validation.

[![Crates.io](https://img.shields.io/crates/v/odds_converter)](https://crates.io/crates/odds_converter)
[![Documentation](https://docs.rs/odds_converter/badge.svg)](https://docs.rs/odds_converter)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Multiple Formats**: Support for American, Decimal, and Fractional odds
- **Bidirectional Conversion**: Convert between any two formats seamlessly
- **Implied Probability**: Calculate implied probabilities from odds
- **String Parsing**: Parse odds from common string representations
- **Robust Validation**: Comprehensive input validation and error handling
- **Zero Dependencies**: Pure Rust implementation with minimal dependencies

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
odds_converter = "0.1.0"
```

## Usage

### Basic Conversions

```rust
use odds_converter::Odds;

// Create odds in different formats
let american = Odds::new_american(150);    // +150 American odds
let decimal = Odds::new_decimal(2.5);      // 2.50 decimal odds
let fractional = Odds::new_fractional(3, 2); // 3/2 fractional odds

// Convert between formats
assert_eq!(american.to_decimal().unwrap(), 2.5);
assert_eq!(decimal.to_american().unwrap(), 150);
assert_eq!(fractional.to_decimal().unwrap(), 2.5);

// Calculate implied probability
assert_eq!(decimal.implied_probability().unwrap(), 0.4); // 40%
```

### String Parsing

```rust
use odds_converter::Odds;

// Parse from strings
let odds1: Odds = "+150".parse().unwrap();    // American format
let odds2: Odds = "2.50".parse().unwrap();    // Decimal format  
let odds3: Odds = "3/2".parse().unwrap();     // Fractional format

// Display as strings
println!("{}", odds1); // "+150"
println!("{}", odds2); // "2.50"
println!("{}", odds3); // "3/2"
```

### Error Handling

```rust
use odds_converter::{Odds, OddsError};

// Validation catches invalid odds
let invalid = Odds::new_decimal(0.5);
match invalid.validate() {
    Err(OddsError::InvalidDecimalOdds(msg)) => {
        println!("Error: {}", msg); // "Decimal odds must be >= 1.0"
    }
    _ => {}
}

// Parsing handles malformed input
let result: Result<Odds, _> = "invalid".parse();
assert!(result.is_err());
```

### Real-World Examples

```rust
use odds_converter::Odds;

// Convert Vegas odds to European format
let vegas_line = Odds::new_american(-110);
let european_odds = vegas_line.to_decimal().unwrap();
println!("European odds: {:.2}", european_odds); // 1.91

// Calculate probability for UK fractional odds
let uk_odds = Odds::new_fractional(9, 4);
let probability = uk_odds.implied_probability().unwrap();
println!("Implied probability: {:.1}%", probability * 100.0); // 30.8%

// Parse user input and convert
let user_input = "+200";
let odds: Odds = user_input.parse().unwrap();
let decimal_equivalent = odds.to_decimal().unwrap();
println!("{} American = {:.2} Decimal", user_input, decimal_equivalent);
```

## Odds Formats

### American Odds (Moneyline)
- **Positive numbers**: Profit on a $100 bet (e.g., +150 = $150 profit)
- **Negative numbers**: Amount to bet to win $100 (e.g., -200 = bet $200 to win $100)
- **Common in**: United States

### Decimal Odds (European)
- **Format**: Total return including stake (e.g., 2.50 = $2.50 return on $1 bet)
- **Range**: Always ≥ 1.0
- **Common in**: Europe, Australia, Canada

### Fractional Odds (UK)
- **Format**: Profit ratio as fraction (e.g., 3/2 = win $3 for every $2 bet)
- **Notation**: numerator/denominator
- **Common in**: United Kingdom, Ireland

## API Reference

### Core Types

- `Odds` - Main struct for holding odds in any format
- `OddsFormat` - Enum representing the three odds formats
- `OddsError` - Error types for validation and parsing failures

### Methods

- `new_american(value: i32)` - Create American odds
- `new_decimal(value: f64)` - Create decimal odds  
- `new_fractional(num: u32, den: u32)` - Create fractional odds
- `to_american()` - Convert to American format
- `to_decimal()` - Convert to decimal format
- `to_fractional()` - Convert to fractional format
- `implied_probability()` - Calculate implied probability
- `validate()` - Validate odds values
- `format()` - Get underlying format

### String Operations

- `parse()` - Parse from string (via `FromStr` trait)
- `to_string()` - Format as string (via `Display` trait)

## Mathematical Accuracy

The library handles floating-point precision carefully and includes comprehensive tests for mathematical correctness:

- Property-based testing ensures conversion accuracy
- Round-trip conversions maintain precision within reasonable tolerances
- Edge cases are properly handled (e.g., very small/large odds)

## Error Handling

Comprehensive error types provide detailed information about failures:

- `InvalidAmericanOdds` - Zero, -100, or out-of-range American odds
- `InvalidDecimalOdds` - Less than 1.0, infinite, or NaN decimal odds
- `InvalidFractionalOdds` - Invalid fractional values
- `ZeroDenominator` - Division by zero in fractions
- `ParseError` - Malformed string input
- `ValueOutOfRange` - Unreasonably large values

## Performance

Conversions are highly optimized with minimal allocations:

- Typical conversion time: ~1.6-2.0 nanoseconds
- Zero-allocation for numeric conversions
- Efficient string parsing with detailed error messages

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Mathematical formulas verified against industry-standard sources
- Extensive testing ensures accuracy across all supported ranges
- Built with Rust's emphasis on safety and performance