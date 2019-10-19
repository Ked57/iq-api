# syntax=docker/dockerfile:1.0-experimental
FROM rust:1.36 as build
WORKDIR /tmp
RUN USER=root cargo new --bin builder
WORKDIR /tmp/builder
COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY src src
RUN         --mount=type=cache,target=../../usr/local/cargo/registry \
            --mount=type=cache,target=target \
            cargo build --release
RUN         --mount=type=cache,target=target cp target/release/iq-api /tmp/iq-api

# our final base
FROM debian:stretch-slim

# for connecting to postgres and TLS hosts
RUN apt-get update -y && apt-get install -y libpq-dev openssl libssl1.0-dev ca-certificates

# copy the build artifact from the build stage
COPY --from=build /tmp/iq-api .

# set the startup command to run your binary
CMD ["./iq-api"]