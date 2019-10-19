FROM ekidd/rust-musl-builder:1.38.0 AS builder

ARG TARGET_ARCHITECTURE=x86_64-unknown-linux-musl

USER root

RUN apt-get update && \
  apt-get install -y upx

WORKDIR /app

RUN sudo chown rust:rust .

USER rust

# Install and build dependencies first to avoid rebuilding on source change.
COPY Cargo.toml .
RUN mkdir src && \
  echo "fn main() { }" > src/main.rs && \
  cargo build --release && \
  rm -rf /app/target/${TARGET_ARCHITECTURE}/release/deps/iq-api*

# Build and compress the program.
COPY . .
RUN cargo build --target ${TARGET_ARCHITECTURE} --release
RUN strip /app/target/${TARGET_ARCHITECTURE}/release/iq-api
RUN upx --brute /app/target/${TARGET_ARCHITECTURE}/release/iq-api
RUN mv /app/target/${TARGET_ARCHITECTURE}/release/iq-api /app/iq-api

FROM scratch

COPY --from=builder /app/iq-api /
EXPOSE 9000

CMD ["/iq-api"]
