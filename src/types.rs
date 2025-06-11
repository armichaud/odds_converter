/// Represents the different formats of betting odds.
///
/// Betting odds can be expressed in three main formats, each common in different regions:
/// - **American odds**: Used primarily in the United States (e.g., +150, -200)
/// - **Decimal odds**: Used in Europe, Australia, and Canada (e.g., 2.50, 1.50)
/// - **Fractional odds**: Traditional format used in the UK (e.g., 3/2, 1/2)
///
/// # Examples
///
/// ```
/// use odds_converter::OddsFormat;
///
/// let american = OddsFormat::American(150);    // +150 American odds
/// let decimal = OddsFormat::Decimal(2.5);      // 2.50 decimal odds  
/// let fractional = OddsFormat::Fractional(3, 2); // 3/2 fractional odds
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum OddsFormat {
    /// American odds format (also known as moneyline odds).
    ///
    /// Positive values indicate the profit on a $100 bet.
    /// Negative values indicate how much you need to bet to win $100.
    ///
    /// # Examples
    /// - `American(150)` means a $100 bet wins $150 profit
    /// - `American(-200)` means you need to bet $200 to win $100 profit
    American(i32),

    /// Decimal odds format (also known as European odds).
    ///
    /// Represents the total return (including stake) for a $1 bet.
    /// Must be greater than or equal to 1.0.
    ///
    /// # Examples  
    /// - `Decimal(2.5)` means a $1 bet returns $2.50 total ($1.50 profit)
    /// - `Decimal(1.5)` means a $1 bet returns $1.50 total ($0.50 profit)
    Decimal(f64),

    /// Fractional odds format (also known as UK odds).
    ///
    /// Represents the ratio of profit to stake as a fraction.
    /// The first value is the numerator (profit), the second is the denominator (stake).
    ///
    /// # Examples
    /// - `Fractional(3, 2)` means 3:2 odds (bet $2 to win $3 profit)
    /// - `Fractional(1, 2)` means 1:2 odds (bet $2 to win $1 profit)
    Fractional(u32, u32),
}

/// The main odds structure that can hold any of the three odds formats.
///
/// This struct provides a unified interface for working with different odds formats,
/// allowing easy conversion between them and calculation of implied probabilities.
///
/// # Examples
///
/// ```
/// use odds_converter::Odds;
///
/// // Create odds in different formats
/// let american = Odds::new_american(150);
/// let decimal = Odds::new_decimal(2.5);
/// let fractional = Odds::new_fractional(3, 2);
///
/// // All represent the same odds
/// assert_eq!(american.to_decimal().unwrap(), 2.5);
/// assert_eq!(decimal.to_american().unwrap(), 150);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Odds {
    pub(crate) format: OddsFormat,
}

impl Odds {
    /// Creates new odds in American format.
    ///
    /// American odds use positive and negative numbers to represent betting odds:
    /// - Positive numbers show profit on a $100 bet
    /// - Negative numbers show amount needed to bet to win $100
    ///
    /// Edge case normalization occurs automatically:
    /// - Positive values 1-99 are converted to equivalent negative odds
    /// - Negative values -1 to -99 are converted to equivalent positive odds
    ///
    /// # Arguments
    ///
    /// * `value` - The American odds value (cannot be 0)
    ///
    /// # Examples
    ///
    /// ```
    /// use odds_converter::Odds;
    ///
    /// let favorite = Odds::new_american(-150);  // Bet $150 to win $100
    /// let underdog = Odds::new_american(200);   // Bet $100 to win $200
    /// let edge_case_pos = Odds::new_american(50);   // Automatically becomes -200
    /// let edge_case_neg = Odds::new_american(-50);  // Automatically becomes +200
    /// ```
    pub fn new_american(value: i32) -> Self {
        use crate::conversions::normalize_american_odds;
        Self {
            format: OddsFormat::American(normalize_american_odds(value)),
        }
    }

    /// Creates new odds in decimal format.
    ///
    /// Decimal odds represent the total return (including original stake) for a unit bet.
    /// A value of 2.0 means even odds (50% probability).
    ///
    /// # Arguments
    ///
    /// * `value` - The decimal odds value (must be >= 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use odds_converter::Odds;
    ///
    /// let even_odds = Odds::new_decimal(2.0);    // 50% probability
    /// let long_odds = Odds::new_decimal(5.0);    // 20% probability
    /// let short_odds = Odds::new_decimal(1.25);  // 80% probability
    /// ```
    pub fn new_decimal(value: f64) -> Self {
        Self {
            format: OddsFormat::Decimal(value),
        }
    }

    /// Creates new odds in fractional format.
    ///
    /// Fractional odds represent the ratio of profit to stake.
    /// For example, 3/2 odds mean you win $3 for every $2 bet.
    ///
    /// # Arguments
    ///
    /// * `numerator` - The profit amount (top of fraction)
    /// * `denominator` - The stake amount (bottom of fraction, cannot be 0)
    ///
    /// # Examples
    ///
    /// ```
    /// use odds_converter::Odds;
    ///
    /// let three_to_two = Odds::new_fractional(3, 2);  // 3:2 odds
    /// let evens = Odds::new_fractional(1, 1);         // Even money
    /// let odds_on = Odds::new_fractional(1, 4);       // 1:4 odds (short odds)
    /// ```
    pub fn new_fractional(numerator: u32, denominator: u32) -> Self {
        Self {
            format: OddsFormat::Fractional(numerator, denominator),
        }
    }

    /// Returns a reference to the underlying odds format.
    ///
    /// This allows you to inspect the specific format and value of the odds
    /// without performing any conversions.
    ///
    /// # Examples
    ///
    /// ```
    /// use odds_converter::{Odds, OddsFormat};
    ///
    /// let odds = Odds::new_american(150);
    /// match odds.format() {
    ///     OddsFormat::American(value) => println!("American odds: {}", value),
    ///     OddsFormat::Decimal(value) => println!("Decimal odds: {}", value),
    ///     OddsFormat::Fractional(num, den) => println!("Fractional odds: {}/{}", num, den),
    /// }
    /// ```
    pub fn format(&self) -> &OddsFormat {
        &self.format
    }
}
