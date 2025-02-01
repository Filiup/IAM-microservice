
FROM rust:alpine3.19

RUN apk update && apk add --no-cache musl-dev && apk add --no-cache make protobuf-dev lldb

RUN adduser -u 1000 -D rust
USER rust

ENV CARGO_HOME=/home/rust/.cargo
ENV PATH=$CARGO_HOME/bin:$PATH

RUN cargo install cargo-watch

COPY ./ .
