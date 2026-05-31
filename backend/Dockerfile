FROM rust:1.95 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin sango-backend --bin temporal_worker

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates openssh-client docker-cli && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/sango-backend /usr/local/bin/sango-backend
COPY --from=builder /app/target/release/temporal_worker /usr/local/bin/temporal_worker
CMD ["sango-backend"]

