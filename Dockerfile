FROM rust:1.75.0-alpine3.19 as base

RUN apk update && \
    apk add musl-dev

WORKDIR /app

COPY ./src/ ./src/
COPY ./Cargo.toml .
COPY ./Cargo.lock .

FROM base as builder

RUN cargo build --release

FROM alpine:3.19

WORKDIR /app

COPY --from=builder /app/target/release/postgresql-controller .

RUN adduser app -D -u 1000 -h /app && \
    chown -R app:app /app

USER app

ENTRYPOINT ["/app/postgresql-controller"]
