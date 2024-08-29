# Start with an Alpine Linux base image for building the application
FROM --platform=linux/amd64 alpine:latest AS build

WORKDIR /app

COPY . .

# Install necessary dependencies and tools
RUN apk add --no-cache \
    curl \
    bash \
    gcc \
    g++ \
    musl-dev \
    openssl-dev \
    libgcc \
    libstdc++ \
    pkgconf \
    make

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . "$HOME/.cargo/env"

# Build the Rust application
RUN export PKG_CONFIG_PATH="/usr/lib/pkgconfig" && \
    export LD_LIBRARY_PATH="/usr/lib" && \
    . "$HOME/.cargo/env" && \
    RUSTFLAGS="-C target-feature=-crt-static" cargo build --release

# Create a new stage for the final image
FROM alpine:latest

WORKDIR /app

# Install necessary runtime libraries
RUN apk add --no-cache \
    libgcc \
    libstdc++ \
    openssl \
    musl

# Copy the built binary from the build stage
COPY --from=build /app/target/release/rnd-helper /app/rnd-helper

# Copy necessary configuration files
COPY --from=build /app/config /app/config
COPY --from=build /app/.env /app/.env

# Command to run the application
CMD ["/app/rnd-helper"]