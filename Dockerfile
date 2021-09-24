FROM rust:1-slim AS builder
WORKDIR /build
COPY ./    ./
RUN cargo build --release

FROM debian:11-slim
WORKDIR /app
RUN mkdir /output
COPY ./create-typings.sh ./
COPY --from=builder /build/target/release/postgres-typescript-generator ./
CMD ["./create-typings.sh"]