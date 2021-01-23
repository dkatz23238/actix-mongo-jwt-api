FROM rust:1.49-slim as planner
WORKDIR /usr/src/api-service
# We only pay the installation cost once, 
# it will be cached from the second build onwards
# To ensure a reproducible build consider pinning 
# the cargo-chef version with `--version X.X.X`
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:1.49-slim as cacher
WORKDIR /usr/src/api-service
RUN cargo install cargo-chef
COPY --from=planner /usr/src/api-service/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path /usr/src/api-service/recipe.json

FROM rust:1.49-slim as builder
WORKDIR /usr/src/api-service
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /usr/src/api-service/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin rust-mongo-example

FROM rust:1.49-slim as runtime
WORKDIR /usr/src/api-service
COPY --from=builder /usr/src/api-service/target/release/rust-mongo-example /usr/local/bin
ENTRYPOINT ["/usr/local/bin/rust-mongo-example"]