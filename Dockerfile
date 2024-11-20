# Use fully qualified image name for Docker compatibility
FROM docker.io/library/rust:1.73

# Set working directory
WORKDIR /app

# Copy the entire project first
COPY . .

# Build the application in release mode
RUN cargo build --release

# Expose the server port
EXPOSE 9000

# Set the entrypoint
ENTRYPOINT ["./target/release/rust_tcp_finance_server"]
CMD ["server"]
