FROM rust:1.43.1

WORKDIR /usr/src/api-service
COPY . .

RUN cargo install --path .
EXPOSE 3000
CMD ["rust-mongo-example"]