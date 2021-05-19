FROM rust:alpine as builder

RUN apk add --no-cache musl-dev

RUN rustup default nightly \
 && rustup update

WORKDIR "/app"

COPY . .

RUN cargo --version

RUN cargo test

RUN cargo build --release

RUN ls target/release

RUN ls target/debug

FROM alpine

WORKDIR "/app/"

COPY --from=builder /app/target/release/address_service \
 /usr/local/bin/

ENTRYPOINT ["/usr/local/bin/address_service"]

EXPOSE 8000