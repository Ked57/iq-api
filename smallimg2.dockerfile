FROM rust:buster as build
WORKDIR /usr/src/iq-api
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-tools
RUN USER=root cargo init
COPY Cargo.toml .
RUN cargo fetch --target x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl
COPY src ./src
RUN find src -exec touch {} \;
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN cargo test --release --target x86_64-unknown-linux-musl

FROM alpine:3.10
RUN addgroup -S iq-api && adduser -S -g iq-api iq-api
COPY --from=build /usr/src/iq-api/target/x86_64-unknown-linux-musl/release/iq-api /bin/
USER iq-api
WORKDIR /home/iq-api
CMD ["/bin/iq-api"]