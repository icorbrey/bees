FROM rust:latest AS builder
WORKDIR /usr/src/app

# Prefetch dependencies
COPY ["Cargo.toml", "Cargo.lock", "**/Cargo.toml", "**/Cargo.lock", "./"]
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo fetch

# Build
COPY . .
RUN rustup component add rustfmt
RUN cargo install --path bees-webhook --locked --root /usr/local

# Runtime
FROM debian:bookworm-slim
COPY --from=builder /usr/local/bin/bees-webhook /usr/local/bin/bees-webhook

ARG BEES_WEBHOOK_PORT=3000
ENV BEES_WEBHOOK_PORT=${BEES_WEBHOOK_PORT}
EXPOSE ${BEES_WEBHOOK_PORT}

CMD ["bees-webhook"]
