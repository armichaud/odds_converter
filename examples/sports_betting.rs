//! Sports betting scenarios using the odds converter.
//!
//! This example demonstrates real-world sports betting applications including:
//! - Different sportsbook line formats
//! - Arbitrage opportunity detection
//! - Expected value calculations
//! - Line shopping comparisons

use odds_converter::Odds;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Sports Betting Applications ===\n");

    // Example 1: Different sportsbook formats
    println!("1. Sportsbook Line Comparison:");
    compare_sportsbook_lines()?;
    println!();

    // Example 2: Arbitrage detection
    println!("2. Arbitrage Opportunity Detection:");
    detect_arbitrage()?;
    println!();

    // Example 3: Expected value calculation
    println!("3. Expected Value Calculation:");
    calculate_expected_value()?;
    println!();

    // Example 4: Probability analysis
    println!("4. Market Probability Analysis:");
    analyze_market_probabilities()?;
    println!();

    Ok(())
}

fn compare_sportsbook_lines() -> Result<(), Box<dyn std::error::Error>> {
    println!("   NFL Game: Team A vs Team B");
    println!("   Different sportsbooks offering different formats:");

    // American sportsbook
    let american_book = Odds::new_american(-110);
    println!(
        "   📱 American Sportsbook: {} (American format)",
        american_book
    );
    println!(
        "      Decimal equivalent: {:.3}",
        american_book.to_decimal()?
    );
    println!(
        "      Implied probability: {:.1}%",
        american_book.implied_probability()? * 100.0
    );

    // European sportsbook
    let european_book = Odds::new_decimal(1.909);
    println!(
        "   🌍 European Sportsbook: {} (Decimal format)",
        european_book
    );
    println!(
        "      American equivalent: {}",
        european_book.to_american()?
    );
    println!(
        "      Implied probability: {:.1}%",
        european_book.implied_probability()? * 100.0
    );

    // UK sportsbook
    let uk_book = Odds::new_fractional(10, 11);
    println!("   🇬🇧 UK Sportsbook: {} (Fractional format)", uk_book);
    println!("      Decimal equivalent: {:.3}", uk_book.to_decimal()?);
    println!(
        "      Implied probability: {:.1}%",
        uk_book.implied_probability()? * 100.0
    );

    Ok(())
}

fn detect_arbitrage() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Basketball Game: Lakers vs Warriors");

    // Different books with different odds on same event
    let book_a_lakers = Odds::new_american(110); // Lakers +110 at Book A
    let book_b_warriors = Odds::new_american(-105); // Warriors -105 at Book B

    let lakers_prob = book_a_lakers.implied_probability()?;
    let warriors_prob = book_b_warriors.implied_probability()?;
    let total_prob = lakers_prob + warriors_prob;

    println!(
        "   Book A - Lakers: {} (prob: {:.1}%)",
        book_a_lakers,
        lakers_prob * 100.0
    );
    println!(
        "   Book B - Warriors: {} (prob: {:.1}%)",
        book_b_warriors,
        warriors_prob * 100.0
    );
    println!("   Total implied probability: {:.1}%", total_prob * 100.0);

    if total_prob < 1.0 {
        let profit_margin = (1.0 - total_prob) * 100.0;
        println!(
            "   🎯 ARBITRAGE OPPORTUNITY! Profit margin: {:.2}%",
            profit_margin
        );

        // Calculate optimal bet sizes for $1000 total
        let total_stake = 1000.0;
        let lakers_stake = total_stake * lakers_prob / total_prob;
        let warriors_stake = total_stake * warriors_prob / total_prob;

        println!("   Optimal bet allocation for $1000:");
        println!("     Lakers: ${:.2}", lakers_stake);
        println!("     Warriors: ${:.2}", warriors_stake);

        let guaranteed_profit = total_stake * (1.0 - total_prob);
        println!("   Guaranteed profit: ${:.2}", guaranteed_profit);
    } else {
        println!(
            "   ❌ No arbitrage opportunity (overround: {:.1}%)",
            (total_prob - 1.0) * 100.0
        );
    }

    Ok(())
}

fn calculate_expected_value() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Tennis Match: Your Analysis vs Market Odds");

    // Your estimated probability vs market odds
    let your_estimate = 0.60; // You think Player A has 60% chance to win
    let market_odds = Odds::new_american(150); // Market offers +150 on Player A
    let market_prob = market_odds.implied_probability()?;

    println!(
        "   Your estimated probability: {:.1}%",
        your_estimate * 100.0
    );
    println!(
        "   Market odds: {} (implied prob: {:.1}%)",
        market_odds,
        market_prob * 100.0
    );

    // Calculate expected value for $100 bet
    let stake = 100.0;
    let payout = market_odds.to_decimal()? * stake;
    let profit = payout - stake;

    let ev = (your_estimate * profit) + ((1.0 - your_estimate) * (-stake));

    println!("   Expected Value calculation for $100 bet:");
    println!(
        "     If win ({:.0}% chance): +${:.2}",
        your_estimate * 100.0,
        profit
    );
    println!(
        "     If lose ({:.0}% chance): -${:.2}",
        (1.0 - your_estimate) * 100.0,
        stake
    );
    println!("     Expected Value: ${:.2}", ev);

    if ev > 0.0 {
        println!("   ✅ POSITIVE EV! This is a good bet according to your analysis.");
        let edge = (your_estimate - market_prob) * 100.0;
        println!("   Your edge: {:.1} percentage points", edge);
    } else {
        println!("   ❌ Negative EV. Avoid this bet according to your analysis.");
    }

    Ok(())
}

fn analyze_market_probabilities() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Horse Racing: 5-Horse Field");

    let horses = vec![
        ("Thunder Bolt", Odds::new_fractional(2, 1)), // 2/1
        ("Speed Demon", Odds::new_fractional(5, 2)),  // 5/2
        ("Lucky Star", Odds::new_american(400)),      // +400
        ("Fast Track", Odds::new_decimal(6.0)),       // 6.0
        ("Wind Runner", Odds::new_american(-150)),    // -150 (favorite)
    ];

    let mut total_prob = 0.0;

    println!("   Horse            Odds        Decimal    Probability");
    println!("   ─────────────────────────────────────────────────────");

    for (horse, odds) in &horses {
        let decimal = odds.to_decimal()?;
        let prob = odds.implied_probability()?;
        total_prob += prob;

        println!(
            "   {:12} {:10} {:7.2}    {:5.1}%",
            horse,
            format!("{}", odds),
            decimal,
            prob * 100.0
        );
    }

    println!("   ─────────────────────────────────────────────────────");
    println!("   Total market probability: {:.1}%", total_prob * 100.0);

    let overround = (total_prob - 1.0) * 100.0;
    println!("   Bookmaker margin (overround): {:.1}%", overround);

    // True probabilities (removing overround)
    println!("\n   True probabilities (removing bookmaker margin):");
    for (horse, odds) in &horses {
        let market_prob = odds.implied_probability()?;
        let true_prob = market_prob / total_prob;
        println!(
            "   {:12} {:.1}% (was {:.1}%)",
            horse,
            true_prob * 100.0,
            market_prob * 100.0
        );
    }

    Ok(())
}
