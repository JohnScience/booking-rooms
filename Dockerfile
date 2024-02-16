FROM rust:latest as builder

RUN apt-get update && \
    apt-get install -y musl-tools musl-dev openssl && \
    rm -rf /var/lib/apt/lists/*

RUN rustup show | grep 'default' | awk -F'-' '{print $2}' > arch.txt
RUN rustup show | grep 'default' | awk -F'-' '{print $2"-unknown-linux-musl"}' > target.txt

RUN rustup target add $(cat /target.txt)

RUN mkdir /app
RUN mkdir /calgary_central_library

ADD app/Cargo.toml ./app
ADD app/src ./app/src
ADD calgary_central_library/Cargo.toml ./calgary_central_library
ADD calgary_central_library/src ./calgary_central_library/src

WORKDIR /app

RUN cargo build -F vendored_ssl --target $(cat /target.txt) --release
RUN mv target/$(cat /target.txt) target/arch-unknown-linux-musl
# ENTRYPOINT [ "sh", "-c", "ls target/$(cat /target.txt)" ]


FROM alpine as runtime
COPY --from=builder /app/target/arch-unknown-linux-musl/release/app /app

ENTRYPOINT ["/app"]
