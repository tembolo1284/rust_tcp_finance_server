use std::net::TcpStream;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use crate::utils::{generate_random_ticker_and_price, format_price, PriceTracker};

pub fn handle_client(mut stream: TcpStream, price_tracker: Arc<Mutex<PriceTracker>>) {
    let mut buffer = [0; 512];
    stream.write_all(b"Connected to Stock Price Server\n").unwrap();
    println!("Client connected: {}", stream.peer_addr().unwrap());
    
    while let Ok(size) = stream.read(&mut buffer) {
        if size == 0 {
            break;
        }
        
        let received = String::from_utf8_lossy(&buffer[..size]).trim().to_string();
        println!("Received: {}", received);
        
        let response = match received.as_str() {
            "list" => {
                let tickers = crate::utils::TICKERS.join(", ");
                format!("Available tickers: {}\n", tickers)
            }
            "quit" => {
                stream.write_all(b"Goodbye!\n").unwrap();
                return;
            }
            query if query.starts_with("stats ") => {
                let ticker = query.strip_prefix("stats ").unwrap().to_uppercase();
                if !crate::utils::TICKERS.contains(&ticker.as_str()) {
                    format!("Invalid ticker: {}. Use 'list' to see available tickers.\n", ticker)
                } else {
                    let tracker = price_tracker.lock().unwrap();
                    match tracker.get_prices(&ticker) {
                        Some(prices) => {
                            let avg = tracker.average(&ticker).unwrap();
                            let std_dev = tracker.std_deviation(&ticker).unwrap();
                            format!(
                                "Stats for {}:\nPrices: {:?}\nAverage: {:.2}\nStd Dev: {:.2}\n",
                                ticker, prices, avg, std_dev
                            )
                        }
                        None => format!("No data available yet for ticker: {}\n", ticker),
                    }
                }
            }
            ticker if crate::utils::TICKERS.contains(&ticker.to_uppercase().as_str()) => {
                let (_, price) = generate_random_ticker_and_price();
                let ticker = ticker.to_uppercase();
                {
                    let mut tracker = price_tracker.lock().unwrap();
                    tracker.add_price(&ticker, price);
                }
                format_price(&ticker, price)
            }
            _ => {
                "Invalid command. Available commands:\n\
                 - list: Show available tickers\n\
                 - stats <ticker>: Show statistics for a ticker\n\
                 - <ticker>: Get current price for a ticker\n\
                 - quit: Disconnect from server\n".to_string()
            }
        };
        
        stream.write_all(response.as_bytes()).unwrap();
    }
    println!("Client disconnected");
}
