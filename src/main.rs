#[macro_use]
extern crate lazy_static;

use actix_web::{middleware, web, App, HttpServer};
mod controller;
use database::get_user_collection;
use mongodb::sync::Client;
use service::users::UserService;
mod database;
mod models;
mod service;
use dotenv;

use env_logger;
use std::env;

pub struct ServiceContainer {
    user: UserService,
}

impl ServiceContainer {
    pub fn new(user: UserService) -> Self {
        ServiceContainer { user }
    }
}

pub struct AppState {
    service_container: ServiceContainer,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let user_collection = get_user_collection();
    HttpServer::new(move || {
        let service_container = ServiceContainer::new(UserService::new(user_collection.clone()));
        let post_user = web::resource("/").route(web::post().to(controller::users::signup_user));
        let authenticate = web::resource("/authenticate")
            .route(web::post().to(controller::users::authenticate_user));

        let get_user =
            web::resource("/{user_id}").route(web::get().to(controller::users::get_single_user));

        let app = App::new()
            .wrap(middleware::Logger::default())
            .data(AppState { service_container })
            .service(post_user)
            .service(authenticate)
            .service(get_user);
        app
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
