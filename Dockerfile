ARG RUST_VERSION=1.78.0

FROM rust:${RUST_VERSION}-slim-bookworm AS builder

WORKDIR /workspace

ADD . .

RUN \
    --mount=type=cache,target=/workspace/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp ./target/release/temper /

FROM debian:bookworm-slim AS runner-builder

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y \
    postgresql

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser

COPY --from=builder /temper /usr/local/bin/temper

RUN chown appuser /usr/local/bin/temper

COPY --from=builder /app/.config /opt/temper/.config

RUN chown -R appuser /opt/temper

USER appuser

ENV DATABASE_URL = "" \
    MODE = "production" \
    RUST_LOG="temper=debug,info"

WORKDIR /opt/pzzld

EXPOSE 8080/tcp

ENTRYPOINT ["temper"]
