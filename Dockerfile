# Use multi-stage build to keep the final image small
FROM rust:1.75 as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/src/zenith

# Copy the Cargo files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

# Now copy the actual source code and build the application
COPY . .
RUN rm src/main.rs  # Remove the dummy file
COPY src src  # Copy actual source

RUN cargo build --release --target x86_64-unknown-linux-musl --locked

# Create the final image
FROM alpine:latest

# Install any runtime dependencies
RUN apk --no-cache add \
    ca-certificates

# Copy the binary from the builder stage
COPY --from=builder /usr/src/zenith/target/x86_64-unknown-linux-musl/release/zenith /usr/local/bin/zenith

# Create a non-root user
RUN addgroup -g 1001 -S zenith && \
    adduser -u 1001 -S zenith -G zenith

# Change ownership of the binary
RUN chown zenith:zenith /usr/local/bin/zenith

# Switch to the non-root user
USER zenith

# Set the entrypoint
ENTRYPOINT ["zenith"]

# Default command
CMD ["--help"]