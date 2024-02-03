FROM rust:1-bookworm as builder

WORKDIR /usr/src/app

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/serve ./serve

# Runtime image
FROM debian:bookworm-slim

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/serve /app/serve
COPY --from=builder /usr/src/app/rhyme.db /app/rhyme.db

# Run the app
CMD ./serve