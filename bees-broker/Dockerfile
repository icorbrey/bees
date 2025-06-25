FROM rust:latest AS builder
WORKDIR /usr/src/app

# Prefetch dependencies
COPY Cargo.toml Cargo.lock */Cargo.toml */Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-index,target=/usr/local/cargo/git \
    cargo fetch

# Copy over source
COPY . .

# Build
RUN rustup component add rustfmt
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-index,target=/usr/local/cargo/git \
    --mount=type=cache,id=build,target=/usr/src/app/target \
    CARGO_INCREMENTAL=1 \
    cargo build --bin bees-broker \
    && mkdir -p /usr/local/bin \
    && cp target/debug/bees-broker /usr/local/bin/bees-broker

# Runtime
FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install binary
COPY --from=builder \
    /usr/local/bin/bees-broker \
    /usr/local/bin/bees-broker

ARG BEES_WEBHOOK_PORT=3000
ENV BEES_WEBHOOK_PORT=${BEES_WEBHOOK_PORT}
EXPOSE ${BEES_WEBHOOK_PORT}

CMD ["bees-broker"]
