# Docker build stage
FROM rust:1.91 AS builder
WORKDIR /app

# Install protoc + deps
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    libprotobuf-dev \
    pkg-config \
    build-essential

COPY . .

# Build release candidate
#RUN cargo install sqlx-cli --no-default-features --features postgres
#RUN cargo sqlx prepare
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/cogito_api /usr/local/bin/cogito_api

RUN apt-get update && apt-get install -y libssl3

EXPOSE 8080
CMD [ "cogito_api" ]
