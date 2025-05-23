# ── Build stage ──────────────────────────────
FROM rust:1.87.0-slim-bookworm AS builder
WORKDIR /app
COPY gateway ./gateway
RUN cargo install --path gateway/crate --locked --root /opt/gateway

# ── Runtime stage ────────────────────────────
FROM debian:bookworm-slim
COPY --from=builder /opt/gateway/bin/llm_gateway /usr/local/bin/
ENV RUST_LOG=info
EXPOSE 8080
CMD ["llm_gateway"]