from  rust:1.80 as builder

WORKDIR /usr/src/myapp

COPY . .

RUN cargo install --path .

# FROM debian:bullseye-slim

# COPY --from=builder /usr/local/cargo/bin/rss-reader /usr/local/bin/rss-reader

CMD ["rss-reader"]
