use actix_web::{middleware, web, App, HttpServer};
use dotenv;
use mongodb::sync::{Client, Collection, Database};

use std::env;

pub fn get_db() -> Database {
    let database_name: String = env::var("MONGO_DB_NAME").unwrap();
    let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
    client.database(&database_name)
}

pub fn get_user_collection() -> Collection {
    get_db().collection("users")
}
