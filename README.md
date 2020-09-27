# Actix Web Service with JWT Authentication and MongoDB

```toml
[dependencies]
actix-web = "2"
actix-rt = "1"
serde = "1.0.103"
futures = "0.3.4"
dotenv = "0.15.0"
env_logger = "0.7"
validator = "0.10"
validator_derive = "0.10"
actix-web-validator = "1.0.0"
bcrypt = "0.8"
jsonwebtoken = "7.2.0"
chrono = "0.4.15"
regex = "1.3.9"
lazy_static = "1.4.0"
```

# Important

This application uses some pregenerated RSA keys. Do not use these keys in production. Make sure to generate new .pem format RSA256 key pair.

# About

This is a simple HTTP API written in Rust using the actix-web framework.

Actix web is a highly performant strongly and soundly typed web framework for building all types of web applications.

For simplicity is uses MongoDB as the database and only uses the sync API's within MongoDB to keep it simple.
Async implementations are welcome via Pull requests.

Designed to be modular and extensible.

# Environment Variables

 - MONGO_DB_NAME=myapp
 - MONGO_COLLECTION_NAME=mycollection
 - PUBLIC_KEY_PATH=./keys/rsa_public.pem
 - PRIVATE_KEY_PATH=./keys/rsa_private.pem
 - RUST_BACKTRACE=1

## Start database locally

```sh
docker run --name mongodb  -p 27017:27017 -e ALLOW_EMPTY_PASSWORD=yes -e MONGODB_EXTRA_FLAGS='--wiredTigerCacheSizeGB=2' bitnami/mongodb:latest
```

## Run Tests

```sh
cargo test
```

## Run Web Server

Restart or use a new database when running as the tests will seed the database with unwanted data.

```sh
cargo run
```
