//! Command-line odds calculator tool.
//!
//! This example demonstrates a practical command-line application for converting
//! betting odds between formats and calculating implied probabilities.
//!
//! Usage: cargo run --example odds_calculator

use odds_converter::{Odds, OddsError};
use std::io::{self, Write};

fn main() {
    println!("=== Betting Odds Calculator ===");
    println!("Enter odds in any format (American: +150/-200, Decimal: 2.50, Fractional: 3/2)");
    println!("Type 'quit' to exit\n");

    loop {
        print!("Enter odds: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.to_lowercase() == "quit" {
                    println!("Goodbye!");
                    break;
                }

                if input.is_empty() {
                    continue;
                }

                match process_odds(input) {
                    Ok(_) => {}
                    Err(e) => println!("Error: {}\n", e),
                }
            }
            Err(error) => println!("Error reading input: {}", error),
        }
    }
}

fn process_odds(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Try to parse the input as odds
    let odds: Odds = input.parse().map_err(|e: OddsError| e)?;

    // Validate the odds
    odds.validate()?;

    println!("\n--- Conversion Results ---");

    // Show all format conversions
    let american = odds.to_american()?;
    let decimal = odds.to_decimal()?;
    let (frac_num, frac_den) = odds.to_fractional()?;
    let probability = odds.implied_probability()?;

    println!("American odds:     {}", format_american(american));
    println!("Decimal odds:      {:.3}", decimal);
    println!("Fractional odds:   {}/{}", frac_num, frac_den);
    println!(
        "Implied probability: {:.2}% ({:.4})",
        probability * 100.0,
        probability
    );

    // Show betting scenarios
    println!("\n--- Betting Scenarios ---");
    show_betting_scenarios(american, decimal, probability);

    println!();
    Ok(())
}

fn format_american(odds: i32) -> String {
    if odds > 0 {
        format!("+{}", odds)
    } else {
        format!("{}", odds)
    }
}

fn show_betting_scenarios(american: i32, decimal: f64, probability: f64) {
    if american > 0 {
        println!("Underdog bet:");
        println!(
            "  - Bet $100, win ${} profit (total return ${})",
            american,
            100 + american
        );
        println!(
            "  - Bet $10, win ${:.2} profit (total return ${:.2})",
            american as f64 / 10.0,
            10.0 + american as f64 / 10.0
        );
    } else {
        let bet_amount = -american;
        println!("Favorite bet:");
        println!(
            "  - Bet ${}, win $100 profit (total return ${})",
            bet_amount,
            bet_amount + 100
        );
        println!(
            "  - Bet $10, win ${:.2} profit (total return ${:.2})",
            1000.0 / bet_amount as f64,
            10.0 + 1000.0 / bet_amount as f64
        );
    }

    println!("Decimal calculation:");
    println!("  - Bet $1, total return ${:.3}", decimal);
    println!("  - Bet $100, total return ${:.2}", decimal * 100.0);

    let breakeven_percentage = probability * 100.0;
    println!("Break-even analysis:");
    println!(
        "  - Need to win {:.1}% of bets to break even long-term",
        breakeven_percentage
    );

    if probability > 0.5 {
        println!("  - This is a FAVORITE (> 50% implied probability)");
    } else if probability < 0.5 {
        println!("  - This is an UNDERDOG (< 50% implied probability)");
    } else {
        println!("  - This is EVEN ODDS (exactly 50% implied probability)");
    }
}
