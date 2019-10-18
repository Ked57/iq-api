FROM rust as cargo-build

WORKDIR /usr/src/iq-api

COPY . .

RUN cargo build --release

EXPOSE $PORT

CMD ["./target/release/iq-api"]