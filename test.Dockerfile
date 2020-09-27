FROM rust:1.43.1

WORKDIR /usr/src/api-service
COPY . .
CMD ["cargo test"]