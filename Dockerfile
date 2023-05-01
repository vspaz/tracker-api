FROM rust:1.69-bullseye

WORKDIR /tracker-api

RUN apt-get update && apt-get upgrade -y \
    && apt-get install -y \
        procps \
        vim \
        less \
        telnet \
        curl \
        net-tools\
        upx-ucl

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/tracker"]