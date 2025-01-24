# Build stage
FROM rust:1.84 as builder

WORKDIR /absint

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY . .

RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

WORKDIR /absint

COPY --from=builder /absint/target/release/rust-absint /absint/
COPY --from=builder /absint/test /absint/test/

ENTRYPOINT [ "/absint/rust-absint" ]
