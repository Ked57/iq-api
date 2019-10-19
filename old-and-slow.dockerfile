FROM rust as cargo-build

WORKDIR /usr/src/iq-api

COPY . .

RUN cargo build --release

EXPOSE 9000

CMD ["./target/release/iq-api"]