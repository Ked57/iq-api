FROM rust:1.36.0-stretch AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    musl-dev \
    musl-tools \
    make \
    g++-multilib \
    zlib1g-dev \
    brotli

WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl
RUN USER=root cargo new iq-api

WORKDIR /usr/src/iq-api

COPY Cargo.toml Cargo.lock ./
RUN \
    LIB_LDFLAGS=-L/usr/lib/x86_64-linux-gnu \
    CFLAGS="-shared -I/usr/include/x86_64-linux-musl -idirafter/usr/include" \
    CXX=g++ \
    CC=musl-gcc \
    cargo build --target x86_64-unknown-linux-musl --release

RUN rm src/*.rs && \
    rm -rf target/x86_64-unknown-linux-musl/release/deps/iq-api*

COPY src ./src
RUN \
    LIB_LDFLAGS=-L/usr/lib/x86_64-linux-gnu \
    CFLAGS="-shared -I/usr/include/x86_64-linux-musl -idirafter/usr/include" \
    CXX=g++ \
    CC=musl-gcc \
    cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

COPY --from=builder /usr/src/iq-api/target/x86_64-unknown-linux-musl/release/iq-api .

USER 1000
CMD ["./iq-api"]