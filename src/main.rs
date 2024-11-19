use rust_tcp_finance_server::{
  config::load_config,
  server,
  client,
};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [server|client]", args[0]);
        return;
    }

    let config = load_config("config/config.toml").expect("Failed to load configuration");

    match args[1].as_str() {
        "server" => {
            server::start_server(config.server.host, config.server.port);
        }
        "client" => {
            client::start_client(&config.server.host, config.server.port);
        }
        _ => {
            println!("Invalid argument. Use 'server' or 'client'.");
        }
    }
}

