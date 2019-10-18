FROM rust as cargo-build

WORKDIR /usr/src/iq-api

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/iq-api"]