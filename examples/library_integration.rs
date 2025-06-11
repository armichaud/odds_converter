//! Library integration examples.
//!
//! This example shows how to integrate the odds_converter library into
//! larger applications, including JSON serialization, database storage,
//! and API integration patterns.

use odds_converter::{Odds, OddsError};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Library Integration Examples ===\n");

    // Example 1: Working with collections
    println!("1. Working with Collections:");
    odds_collections_example()?;
    println!();

    // Example 2: Batch processing
    println!("2. Batch Processing:");
    batch_processing_example()?;
    println!();

    // Example 3: Error handling patterns
    println!("3. Error Handling Patterns:");
    error_handling_patterns()?;
    println!();

    // Example 4: Custom data structures
    println!("4. Custom Data Structures:");
    custom_structures_example()?;
    println!();

    // Example 5: Functional programming patterns
    println!("5. Functional Programming:");
    functional_patterns_example()?;
    println!();

    Ok(())
}

fn odds_collections_example() -> Result<(), Box<dyn std::error::Error>> {
    // Store odds for multiple games
    let mut games: HashMap<String, Vec<Odds>> = HashMap::new();

    // NFL games
    games.insert(
        "NFL_Game1".to_string(),
        vec![
            Odds::new_american(-110), // Team A
            Odds::new_american(-110), // Team B
        ],
    );

    // Premier League (using decimal odds)
    games.insert(
        "PL_Game1".to_string(),
        vec![
            Odds::new_decimal(2.10), // Home win
            Odds::new_decimal(3.25), // Draw
            Odds::new_decimal(3.80), // Away win
        ],
    );

    // Horse racing (using fractional odds)
    games.insert(
        "Race1".to_string(),
        vec![
            Odds::new_fractional(2, 1),  // Horse 1
            Odds::new_fractional(5, 2),  // Horse 2
            Odds::new_fractional(8, 1),  // Horse 3
            Odds::new_fractional(12, 1), // Horse 4
        ],
    );

    // Process all games
    for (game_id, odds_list) in &games {
        println!("   Game: {}", game_id);
        for (i, odds) in odds_list.iter().enumerate() {
            let prob = odds.implied_probability()?;
            println!(
                "     Option {}: {} (prob: {:.1}%)",
                i + 1,
                odds,
                prob * 100.0
            );
        }

        // Calculate total probability
        let total_prob: f64 = odds_list
            .iter()
            .map(|o| o.implied_probability().unwrap_or(0.0))
            .sum();
        println!("     Total probability: {:.1}%", total_prob * 100.0);
        println!();
    }

    Ok(())
}

fn batch_processing_example() -> Result<(), Box<dyn std::error::Error>> {
    // Simulate processing odds from different sources
    let odds_strings = vec![
        "+150", "-200", "2.50", "3/2", "1.91", "+300", "-110", "4.5", "9/4", "1.67",
    ];

    println!(
        "   Processing {} odds from various sources:",
        odds_strings.len()
    );

    // Parse all odds, collecting successes and failures
    let results: Vec<Result<Odds, OddsError>> =
        odds_strings.iter().map(|s| s.parse::<Odds>()).collect();

    let successful_odds: Vec<&Odds> = results.iter().filter_map(|r| r.as_ref().ok()).collect();

    let failed_parses: Vec<&OddsError> = results.iter().filter_map(|r| r.as_ref().err()).collect();

    println!(
        "   Successfully parsed: {}/{}",
        successful_odds.len(),
        odds_strings.len()
    );
    println!("   Failed to parse: {}", failed_parses.len());

    // Convert all successful odds to decimal for comparison
    println!("\n   Converted to decimal format:");
    for (i, odds) in successful_odds.iter().enumerate() {
        let decimal = odds.to_decimal()?;
        let original = odds_strings[i];
        println!("     {} -> {:.3}", original, decimal);
    }

    // Find the best and worst odds
    if let (Some(best), Some(worst)) = find_best_worst_odds(&successful_odds)? {
        println!("\n   Best odds (highest return): {:.3}", best);
        println!("   Worst odds (lowest return): {:.3}", worst);
    }

    Ok(())
}

fn find_best_worst_odds(
    odds: &[&Odds],
) -> Result<(Option<f64>, Option<f64>), Box<dyn std::error::Error>> {
    if odds.is_empty() {
        return Ok((None, None));
    }

    let decimals: Result<Vec<f64>, _> = odds.iter().map(|o| o.to_decimal()).collect();

    let decimals = decimals?;

    let best = decimals.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let worst = decimals.iter().fold(f64::INFINITY, |a, &b| a.min(b));

    Ok((Some(best), Some(worst)))
}

fn error_handling_patterns() -> Result<(), Box<dyn std::error::Error>> {
    let test_inputs = vec![
        "150",     // Valid American (missing +)
        "+200",    // Valid American
        "0",       // Invalid American (zero)
        "2.5",     // Valid decimal
        "0.5",     // Invalid decimal (< 1.0)
        "3/2",     // Valid fractional
        "3/0",     // Invalid fractional (zero denominator)
        "invalid", // Invalid format
    ];

    println!("   Testing error handling patterns:");

    for input in test_inputs {
        match input.parse::<Odds>() {
            Ok(odds) => match odds.validate() {
                Ok(_) => {
                    let decimal = odds.to_decimal()?;
                    println!("   ✅ '{}' -> {:.3} decimal", input, decimal);
                }
                Err(validation_error) => {
                    println!(
                        "   ❌ '{}' -> Parse OK, but validation failed: {}",
                        input, validation_error
                    );
                }
            },
            Err(parse_error) => {
                println!("   ❌ '{}' -> Parse failed: {}", input, parse_error);
            }
        }
    }

    Ok(())
}

// Custom data structure for a betting market
#[derive(Debug)]
struct BettingMarket {
    description: String,
    outcomes: Vec<MarketOutcome>,
}

#[derive(Debug)]
struct MarketOutcome {
    name: String,
    odds: Odds,
    is_favorite: bool,
}

impl BettingMarket {
    fn new(description: String) -> Self {
        Self {
            description,
            outcomes: Vec::new(),
        }
    }

    fn add_outcome(&mut self, name: String, odds: Odds) -> Result<(), Box<dyn std::error::Error>> {
        odds.validate()?;

        let is_favorite = odds.implied_probability()? > 0.5;

        self.outcomes.push(MarketOutcome {
            name,
            odds,
            is_favorite,
        });

        Ok(())
    }

    fn total_probability(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let mut total = 0.0;
        for outcome in &self.outcomes {
            total += outcome.odds.implied_probability()?;
        }
        Ok(total)
    }

    fn get_favorite(&self) -> Option<&MarketOutcome> {
        self.outcomes.iter().min_by(|a, b| {
            a.odds
                .to_decimal()
                .unwrap_or(f64::INFINITY)
                .partial_cmp(&b.odds.to_decimal().unwrap_or(f64::INFINITY))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

fn custom_structures_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut market = BettingMarket::new(
        "Manchester City vs Liverpool".to_string(),
    );

    market.add_outcome("Manchester City Win".to_string(), Odds::new_decimal(2.10))?;
    market.add_outcome("Draw".to_string(), Odds::new_decimal(3.40))?;
    market.add_outcome("Liverpool Win".to_string(), Odds::new_decimal(3.75))?;

    println!("   Market: {}", market.description);
    println!(
        "   Total probability: {:.1}%",
        market.total_probability()? * 100.0
    );

    if let Some(favorite) = market.get_favorite() {
        println!("   Favorite: {} at {}", favorite.name, favorite.odds);
    }

    println!("   All outcomes:");
    for outcome in &market.outcomes {
        let prob = outcome.odds.implied_probability()?;
        let status = if outcome.is_favorite {
            " (favorite)"
        } else {
            ""
        };
        println!(
            "     {} - {} ({:.1}%){}",
            outcome.name,
            outcome.odds,
            prob * 100.0,
            status
        );
    }

    Ok(())
}

fn functional_patterns_example() -> Result<(), Box<dyn std::error::Error>> {
    let odds_data = vec![
        ("Game 1", vec!["+110", "-130"]),
        ("Game 2", vec!["2.1", "1.8"]),
        ("Game 3", vec!["3/2", "1/2"]),
    ];

    println!("   Functional processing of odds data:");

    // Chain operations: parse -> validate -> convert -> analyze
    let analysis_results: Result<Vec<_>, Box<dyn std::error::Error>> = odds_data
        .iter()
        .map(|(game, odds_strings)| {
            let parsed_odds: Result<Vec<Odds>, _> =
                odds_strings.iter().map(|s| s.parse::<Odds>()).collect();

            let parsed_odds = parsed_odds?;

            // Validate all odds
            for odds in &parsed_odds {
                odds.validate()?;
            }

            // Convert to decimal and find favorite
            let decimal_odds: Result<Vec<f64>, _> =
                parsed_odds.iter().map(|o| o.to_decimal()).collect();

            let decimals = decimal_odds?;
            let min_decimal = decimals.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let total_prob: Result<f64, _> =
                parsed_odds.iter().map(|o| o.implied_probability()).sum();

            Ok((game, min_decimal, total_prob?))
        })
        .collect();

    let results = analysis_results?;

    for (game, favorite_decimal, total_prob) in results {
        println!(
            "   {}: Favorite at {:.2}, Total prob: {:.1}%",
            game,
            favorite_decimal,
            total_prob * 100.0
        );
    }

    Ok(())
}
