FROM rust:1.69-bullseye

WORKDIR /tracker-api

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/tracker"]