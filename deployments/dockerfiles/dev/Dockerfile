FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 

RUN apt-get update
RUN apt-get install -y protobuf-compiler libprotobuf-dev

COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build -p server

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime
RUN apt-get update
RUN apt-get install -y build-essential ca-certificates
WORKDIR /app
EXPOSE 8123
COPY --from=builder /app/target/debug/server /usr/local/bin
ENTRYPOINT ["/usr/local/bin/server"]