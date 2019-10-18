FROM rust as cargo-build

WORKDIR /usr/src/iq-api

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/iq-api*

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/iq-api"]