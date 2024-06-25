# FROM liuchong/rustup:nightly-musl AS builder
FROM rustlang/rust:nightly-bullseye-slim AS builder
WORKDIR /rode-be-build
COPY . .
# RUN rustup install nightly
ARG DB_URL
ENV DATABASE_URL=${DB_URL}
RUN cargo build --release

FROM ubuntu:22.04 AS final
WORKDIR /rode-be-worker
COPY --from=builder /rode-be-build/target/release/rode-be-worker /rode-be-worker
RUN apt-get update \
    && apt-get install curl build-essential -y \
    && apt-get clean
    
ENTRYPOINT ["/rode-be-worker/rode-be-worker"]
