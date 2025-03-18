FROM rust:1.81-slim as builder

# Install OpenSSL development packages
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY . .

# Build the release version
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/holder .
COPY --from=builder /usr/src/app/target/release/verifier .

RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

ENV RUST_LOG=info

EXPOSE 8080