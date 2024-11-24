# RustTCPFinanceServer

## Overview
**RustTCPFinanceServer** is a multithreaded TCP server written in Rust. It simulates stock price streaming for predefined tickers (`GOOG`, `AMZN`, `MSFT`, `TSLA`, and `NFLX`) and allows clients to fetch real-time prices. The server also tracks historical prices and provides statistics such as average price and standard deviation upon request.

## Features
- **Multithreaded Server**: Handles multiple clients simultaneously
- **Simulated Stock Prices**: Streams random prices between 85 and 115 for predefined tickers
- **Historical Data Tracking**: Tracks all prices for each ticker
- **Statistics**: Provides average and standard deviation for requested tickers
- **Configurable Server**: Reads host and port information from `config/config.toml`
- **Dockerized Deployment**: Easily run the server in a containerized environment

## Folder Structure
\```plaintext
RustTCPFinanceServer/

│

├── src/

│   ├── main.rs              # Main entry point

│   ├── server.rs            # Server implementation

│   ├── client_handler.rs    # Client request handling logic

│   ├── client.rs            # Client implementation

│   ├── config.rs            # Configuration file parser

│   ├── utils.rs             # Utility functions (price tracker and random generator)

│   └── lib.rs               # Library interface

│

├── config/

│   └── config.toml          # Configuration file for server host and port

│

├── tests/

│   └── utils_tests.rs       # Unit tests for utility functions

│

├── Dockerfile               # Docker configuration

├── README.md               # Project documentation

├── Cargo.toml              # Rust package configuration

└── .github/

    └── workflows/
        └── rust.yml        # GitHub Actions CI/CD configuration
\```

## Prerequisites
- Rust: Install Rust via [rustup](https://rustup.rs/)
- Docker (optional): Install Docker for containerized deployment
- Netcat (optional): Install netcat for simple TCP client testing

## Installation

1. Clone the repository:
```
git clone https://github.com/tembolo1284/rust_tcp_finance_server.git

cd rust_tcp_finance_server
```

2. Build the project:
```
cargo build --release
```

The executable will be located at `target/release/rust_tcp_finance_server`

## Running the Application

### Starting the Server
```
cargo run --release -- server
```

The server will start and listen for connections on the host and port specified in `config/config.toml` (default: `0.0.0.0:9000`)

### Starting the Client
```
cargo run --release -- client
```

### Available Commands
- `list` - Get a list of available tickers
- `stats <ticker>` - Get statistics for a specific ticker (e.g., `stats GOOG`)
- `<ticker>` - Get current price for a specific ticker (e.g., `AMZN`)
- `quit` or `exit` - Disconnect from the server

## Docker Deployment

1. Build the Docker image:
```
docker build -t rust-tcp-finance-server .
```

2. Run the container (server):
```
docker run -p 9000:9000 rust-tcp-finance-server server
```
3. Can also run a detached container so it runs in the background (server):

```
docker run -d -p 9000:9000 rust-tcp-finance-server server

```

4. Run the client:

```
docker run rust-tcp-finance-server client
```

Useful docker commands to see what the server is doing, and to stop it.

```
docker ps

docker logs <container-id>

docker stop <container-id>

```
## Testing

Run the test suite:
```
cargo test
```

## Configuration

The server configuration is stored in `config/config.toml`:
```toml
[server]
host = "0.0.0.0"
port = 9000
```
# Steps to start both server and client through docker commands:

## 1. First stop any running containers
docker stop $(docker ps -a -q)

### See all containers with details
docker ps -a

### See just the container IDs
docker ps -a -q

### Stop specific containers manually (equivalent to what the combined command does)
docker stop <container-id1> <container-id2> <container-id3>

## 2. Remove any existing containers with same names (to avoid conflicts)
docker rm finance-server 2>/dev/null || true

## 3. Verify the finance-net network exists
docker network ls | grep finance-net

## If it doesn't exist, create it:
## docker network create finance-net

## 4. Start the server container
docker run -d --name finance-server --network finance-net -p 9000:9000 rust-tcp-finance-server server


## 5. Verify server is running
docker ps | grep finance-server

## 6. Get the server's IP address
docker inspect finance-server | grep IPAddress

## 7. Run the client container
docker run --network finance-net rust-tcp-finance-server client

if the above doesn't work one can also try:

## 8. Start client interactively (in another terminal)
docker run -it --network host rust-tcp-finance-server client

# To see logs of finance-server:

```
docker logs finance-server
```

## Author

Paul Nikholas Lopez - [nik.lopez381@gmail.com](mailto:nik.lopez381@gmail.com)

