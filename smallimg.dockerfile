# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

FROM liuchong/rustup

RUN mkdir /usr/app

WORKDIR /usr/app

COPY . .

RUN cargo build --release

CMD ["./target/release/iq-api"]