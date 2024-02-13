FROM rust:latest as builder

RUN apt-get update && \
    apt-get install -y musl-tools musl-dev openssl && \
    rm -rf /var/lib/apt/lists/*

RUN rustup show | grep 'default' | awk -F'-' '{print $2}' > arch.txt
RUN rustup show | grep 'default' | awk -F'-' '{print $2"-unknown-linux-musl"}' > target.txt

RUN rustup target add $(cat /target.txt)

ADD Cargo.toml ./
ADD src src
RUN cargo build -F vendored_ssl --target $(cat /target.txt) --release
RUN mv target/$(cat /target.txt) /target/arch-unknown-linux-musl
#ENTRYPOINT [ "sh", "-c", "ls target/" ]


FROM alpine as runtime
COPY --from=builder /target/arch-unknown-linux-musl/release/app /app

ENTRYPOINT ["/app"]
