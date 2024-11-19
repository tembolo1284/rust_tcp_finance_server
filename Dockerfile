# Use fully qualified image name for Podman compatibility
FROM docker.io/library/rust:1.73

# Set working directory
WORKDIR /app

# Copy only necessary files first
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Now copy the actual source code
COPY . .

# Build the project in release mode
RUN cargo build --release

# Expose the server port
EXPOSE 9000

# Set the entrypoint with the server argument
ENTRYPOINT ["./target/release/rust_tcp_finance_server"]
CMD ["server"]
