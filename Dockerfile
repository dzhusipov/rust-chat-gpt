FROM --platform=linux/amd64 alpine:latest AS build

WORKDIR /app

COPY . .

# Install necessary tools and libraries
# RUN apk add --no-cache \
#     curl \
#     libgcc \
#     libstdc++ \
#     alsa-lib \
#     alsa-lib-dev \
#     pulseaudio \
#     dbus \
#     gcc \
#     libc-dev \
#     pkgconf \
#     libx11-dev \
#     eudev-dev

# Install Rust using rustup and source the cargo environment
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . "$HOME/.cargo/env"


# Build the application
RUN export PKG_CONFIG_PATH="/usr/lib/pkgconfig" && \
    export LD_LIBRARY_PATH="/usr/lib" && \
    . "$HOME/.cargo/env" && \
    RUSTFLAGS="-ldylib=asound" cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=build /app/target/release/rnd-helper /app/rnd-helper

# Copy any necessary configuration files
COPY --from=build /app/config /app/config
COPY --from=build /app/.env /app/.env
# COPY --from=build /app/tmp /app/tmp

# Expose the necessary ports
# EXPOSE 9292

# Command to run the application
CMD ["/app/rnd-helper"]