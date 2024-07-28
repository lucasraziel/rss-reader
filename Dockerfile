from  rust:1.80 as builder

WORKDIR /usr/src/myapp

RUN cargo install cargo-watch

COPY . .
