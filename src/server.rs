use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::client_handler;

pub fn start_server(host: String, port: u16) {
    let address = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&address).expect("Could not bind to address");
    println!("Server running on {}", address);

    let price_tracker = Arc::new(Mutex::new(crate::utils::PriceTracker::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let price_tracker = Arc::clone(&price_tracker);
                thread::spawn(move || {
                    client_handler::handle_client(stream, price_tracker);
                });
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
}

