FROM rust:1.83-bullseye

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

COPY . ./

RUN cargo build --release && rm -rf src

ENV LOG_LEVEL=info
ENV LOG_FORMAT="%Y-%m-%d %H:%M:%S.%f"

EXPOSE 9000

CMD ["./target/release/tracker"]