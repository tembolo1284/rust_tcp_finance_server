use std::io::{self, BufRead, Read, Write};
use std::net::TcpStream;
use std::str;

pub fn start_client(host: &str, port: u16) {
    let address = format!("{}:{}", host, port);
    match TcpStream::connect(&address) {
        Ok(mut stream) => {
            println!("Connected to server at {}", address);
            
            // Read initial server greeting
            let mut response = vec![0; 1024];
            match stream.read(&mut response) {
                Ok(size) if size > 0 => {
                    if let Ok(response_str) = str::from_utf8(&response[..size]) {
                        println!("{}", response_str);
                    }
                }
                _ => {}
            }

            let stdin = io::stdin();
            let mut reader = stdin.lock();
            let mut buffer = String::new();

            println!("\nAvailable commands:");
            println!("- list: Show available tickers");
            println!("- stats <ticker>: Show statistics for a ticker");
            println!("- <ticker>: Get current price for a ticker");
            println!("- quit or exit: Disconnect from server\n");

            loop {
                println!("Enter a command:");
                buffer.clear();
                reader.read_line(&mut buffer).expect("Failed to read from stdin");
                
                let command = buffer.trim();
                if command.is_empty() {
                    continue;
                }

                if command.eq_ignore_ascii_case("quit") || command.eq_ignore_ascii_case("exit") {
                    println!("Disconnecting from server...");
                    if let Err(e) = stream.write_all(b"quit\n") {
                        eprintln!("Failed to send quit command: {}", e);
                    }
                    break;
                }

                if let Err(e) = stream.write_all(command.as_bytes()) {
                    eprintln!("Failed to write to server: {}", e);
                    break;
                }

                let mut response = vec![0; 1024];
                match stream.read(&mut response) {
                    Ok(0) => {
                        println!("Server closed the connection.");
                        break;
                    }
                    Ok(size) => {
                        match str::from_utf8(&response[..size]) {
                            Ok(response_str) => println!("Server response:\n{}", response_str),
                            Err(_) => println!("Received invalid UTF-8 response from server"),
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read response: {}", e);
                        break;
                    }
                }
            }
            println!("Connection closed.");
        }
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
        }
    }
}
