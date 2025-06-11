//! Basic usage examples for the odds_converter library.
//!
//! This example demonstrates the fundamental operations:
//! - Creating odds in different formats
//! - Converting between formats
//! - Calculating implied probabilities
//! - Basic error handling

use odds_converter::{Odds, OddsError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Odds Converter Basic Usage Examples ===\n");

    // Example 1: Creating odds in different formats
    println!("1. Creating odds in different formats:");
    let american_odds = Odds::new_american(150);
    let decimal_odds = Odds::new_decimal(2.5);
    let fractional_odds = Odds::new_fractional(3, 2);

    println!("   American: {}", american_odds);
    println!("   Decimal: {}", decimal_odds);
    println!("   Fractional: {}", fractional_odds);
    println!();

    // Example 2: Converting between formats
    println!("2. Converting between formats:");
    println!(
        "   American +150 to decimal: {:.2}",
        american_odds.to_decimal()?
    );
    println!(
        "   Decimal 2.5 to American: {}",
        decimal_odds.to_american()?
    );
    println!(
        "   Fractional 3/2 to decimal: {:.2}",
        fractional_odds.to_decimal()?
    );
    println!();

    // Example 3: Implied probabilities
    println!("3. Calculating implied probabilities:");
    let prob_american = american_odds.implied_probability()?;
    let prob_decimal = decimal_odds.implied_probability()?;
    let prob_fractional = fractional_odds.implied_probability()?;

    println!(
        "   American +150 probability: {:.1}%",
        prob_american * 100.0
    );
    println!("   Decimal 2.5 probability: {:.1}%", prob_decimal * 100.0);
    println!(
        "   Fractional 3/2 probability: {:.1}%",
        prob_fractional * 100.0
    );
    println!();

    // Example 4: String parsing
    println!("4. Parsing odds from strings:");
    let parsed_american: Odds = "+200".parse()?;
    let parsed_decimal: Odds = "1.75".parse()?;
    let parsed_fractional: Odds = "5/4".parse()?;

    println!(
        "   Parsed '+200': {} (decimal: {:.2})",
        parsed_american,
        parsed_american.to_decimal()?
    );
    println!(
        "   Parsed '1.75': {} (American: {})",
        parsed_decimal,
        parsed_decimal.to_american()?
    );
    println!(
        "   Parsed '5/4': {} (decimal: {:.2})",
        parsed_fractional,
        parsed_fractional.to_decimal()?
    );
    println!();

    // Example 5: Error handling
    println!("5. Error handling examples:");

    // Invalid decimal odds (less than 1.0)
    let invalid_decimal = Odds::new_decimal(0.5);
    if let Err(OddsError::InvalidDecimalOdds(msg)) = invalid_decimal.validate() {
        println!("   Invalid decimal odds: {}", msg);
    }

    // Invalid American odds (zero)
    let invalid_american = Odds::new_american(0);
    if let Err(OddsError::InvalidAmericanOdds(msg)) = invalid_american.validate() {
        println!("   Invalid American odds: {}", msg);
    }

    // Parsing error
    if let Err(OddsError::ParseError(msg)) = "invalid_odds".parse::<Odds>() {
        println!("   Parse error: {}", msg);
    }
    println!();

    // Example 6: American odds edge case handling
    println!("6. American odds edge case handling:");
    println!("   Odds between -99 to +99 are automatically normalized:");
    
    // Positive edge cases
    let edge_case_pos50 = Odds::new_american(50);
    println!("   Input +50 becomes: {}", edge_case_pos50);
    println!("   Decimal equivalent: {:.3}", edge_case_pos50.to_decimal()?);
    
    let edge_case_pos25 = Odds::new_american(25);
    println!("   Input +25 becomes: {}", edge_case_pos25);
    println!("   Decimal equivalent: {:.3}", edge_case_pos25.to_decimal()?);
    
    // Negative edge cases  
    let edge_case_neg50 = Odds::new_american(-50);
    println!("   Input -50 becomes: {}", edge_case_neg50);
    println!("   Decimal equivalent: {:.3}", edge_case_neg50.to_decimal()?);
    
    let edge_case_neg25 = Odds::new_american(-25);
    println!("   Input -25 becomes: {}", edge_case_neg25);
    println!("   Decimal equivalent: {:.3}", edge_case_neg25.to_decimal()?);
    println!();

    // Example 7: Real-world scenario
    println!("7. Real-world betting scenario:");
    println!("   You see odds of -110 on a sports bet (typical American sportsbook line)");
    let sportsbook_odds = Odds::new_american(-110);
    let decimal_equiv = sportsbook_odds.to_decimal()?;
    let implied_prob = sportsbook_odds.implied_probability()?;
    let breakeven_rate = implied_prob * 100.0;

    println!("   American: {}", sportsbook_odds);
    println!("   Decimal equivalent: {:.3}", decimal_equiv);
    println!("   Implied probability: {:.1}%", implied_prob * 100.0);
    println!(
        "   You need to win {:.1}% of the time to break even",
        breakeven_rate
    );

    Ok(())
}
