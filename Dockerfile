FROM rust:latest as builder
RUN apt-get update && \
    apt-get install -y musl-tools openssl git && \
    rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /
ARG CACHEBUST=1
RUN git clone -b debugging https://github.com/JohnScience/webdriver-downloader

WORKDIR /webdriver-downloader

RUN git checkout debugging

WORKDIR /webdriver-downloader/webdriver-downloader-cli
RUN cargo build --no-default-features -F rustls-tls --target x86_64-unknown-linux-musl --release

WORKDIR /app
RUN cargo init
COPY app/Cargo.toml app/Cargo.lock ./
RUN cargo build --target x86_64-unknown-linux-musl --release
COPY app .
RUN cargo build --target x86_64-unknown-linux-musl --release
# (Optional) Remove debug symbols
RUN strip target/x86_64-unknown-linux-musl/release/app

# Use a slim image for running the application
FROM alpine as runtime

RUN apk add chromium

# TODO: consider using jq to parse the JSON from https://github.com/GoogleChromeLabs/chrome-for-testing#json-api-endpoints

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/app /bin/app
COPY --from=builder /webdriver-downloader/target/x86_64-unknown-linux-musl/release/webdriver-downloader /bin/webdriver-downloader

RUN webdriver-downloader --type chrome --driver /bin/chromedriver

CMD ["chromedriver", "--version"]
# CMD ["webdriver-downloader", "--help"]
