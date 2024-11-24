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

# Create a script to handle launching either client or server
RUN echo '#!/bin/bash\n\
if [ "$1" = "client" ]; then\n\
    exec ./target/release/rust_tcp_finance_server client\n\
elif [ "$1" = "server" ]; then\n\
    exec ./target/release/rust_tcp_finance_server server\n\
else\n\
    echo "Please specify either client or server"\n\
    exit 1\n\
fi' > /app/entrypoint.sh && chmod +x /app/entrypoint.sh

# Set the entrypoint to our script
ENTRYPOINT ["/app/entrypoint.sh"]

# Default to server mode if no argument is provided
CMD ["server"]
