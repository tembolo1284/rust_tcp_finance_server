use rand::Rng;
use std::collections::HashMap;

pub const TICKERS: [&str; 5] = ["GOOG", "AMZN", "MSFT", "TSLA", "NFLX"];

/// Generates a random ticker and price between 85 and 115.
pub fn generate_random_ticker_and_price() -> (String, f64) {
    let mut rng = rand::thread_rng();
    let ticker = TICKERS[rng.gen_range(0..TICKERS.len())].to_string();
    let price = rng.gen_range(85.0..115.0);
    (ticker, price)
}

/// Formats the ticker and price for display.
pub fn format_price(ticker: &str, price: f64) -> String {
    format!("Price of {}: ${:.2}\n", ticker, price)
}

/// Stores and manages price data for tickers.
pub struct PriceTracker {
    data: HashMap<String, Vec<f64>>,
}

impl PriceTracker {
    pub fn new() -> Self {
        PriceTracker {
            data: HashMap::new(),
        }
    }

    /// Adds a price for the given ticker.
    pub fn add_price(&mut self, ticker: &str, price: f64) {
        self.data.entry(ticker.to_string()).or_default().push(price);
    }

    /// Calculates the average price for the given ticker.
    pub fn average(&self, ticker: &str) -> Option<f64> {
        self.data.get(ticker).map(|prices| {
            let sum: f64 = prices.iter().sum();
            sum / prices.len() as f64
        })
    }

    /// Calculates the standard deviation for the given ticker.
    pub fn std_deviation(&self, ticker: &str) -> Option<f64> {
        self.data.get(ticker).map(|prices| {
            let avg = self.average(ticker).unwrap();
            let variance = prices.iter().map(|p| (p - avg).powi(2)).sum::<f64>() / prices.len() as f64;
            variance.sqrt()
        })
    }

    /// Returns all prices for the given ticker.
    pub fn get_prices(&self, ticker: &str) -> Option<&Vec<f64>> {
        self.data.get(ticker)
    }
}

