use rust_tcp_finance_server::utils::{PriceTracker, TICKERS};

#[test]
fn test_price_tracker_add_price() {
    let mut tracker = PriceTracker::new();
    tracker.add_price("GOOG", 100.0);
    tracker.add_price("GOOG", 110.0);

    let prices = tracker.get_prices("GOOG").unwrap();
    assert_eq!(prices.len(), 2);
    assert_eq!(prices[0], 100.0);
    assert_eq!(prices[1], 110.0);
}

#[test]
fn test_price_tracker_average() {
    let mut tracker = PriceTracker::new();
    tracker.add_price("GOOG", 100.0);
    tracker.add_price("GOOG", 110.0);

    let avg = tracker.average("GOOG").unwrap();
    assert_eq!(avg, 105.0);
}

#[test]
fn test_price_tracker_std_deviation() {
    let mut tracker = PriceTracker::new();
    tracker.add_price("GOOG", 100.0);
    tracker.add_price("GOOG", 110.0);

    let std_dev = tracker.std_deviation("GOOG").unwrap();
    assert!((std_dev - 5.0).abs() < f64::EPSILON); // Allow floating-point tolerance
}

#[test]
fn test_price_tracker_no_prices() {
    let tracker = PriceTracker::new();
    assert!(tracker.get_prices("GOOG").is_none());
    assert!(tracker.average("GOOG").is_none());
    assert!(tracker.std_deviation("GOOG").is_none());
}

#[test]
fn test_tickers_array() {
    assert_eq!(TICKERS.len(), 5);
    assert!(TICKERS.contains(&"GOOG"));
    assert!(TICKERS.contains(&"NFLX"));
}

#[test]
fn test_tickers_random_price_generation() {
    use rust_tcp_finance_server::utils::generate_random_ticker_and_price;

    let (ticker, price) = generate_random_ticker_and_price();
    assert!(TICKERS.contains(&ticker.as_str()));
    assert!(price >= 85.0 && price <= 115.0);
}

