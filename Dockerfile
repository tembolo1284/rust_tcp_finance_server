# Use Rust official image as base
FROM rust:1.73

# Set working directory
WORKDIR /app

# Copy project files
COPY . .

# Build the project in release mode
RUN cargo build --release

# Expose the server port
EXPOSE 9000

# Run the server
CMD ["./target/release/rust_tcp_finance_server"]

