FROM rust:latest as builder

WORKDIR /usr/src/app

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/serve ./serve

# Runtime image
FROM alpine:latest

# Run as "app" user
RUN adduser -D app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/serve /app/serve
COPY --from=builder /usr/src/app/rhyme.db /app/rhyme.db

# Run the app
CMD ./serve