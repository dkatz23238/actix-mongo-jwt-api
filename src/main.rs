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
        let signup_service =
            web::resource("/users").route(web::post().to(controller::users::signup_user));

        let authenticate_service = web::resource("/authenticate")
            .route(web::post().to(controller::users::authenticate_user));
        let users_resource = web::resource("/users/{user_id}");

        let users_services = users_resource
            .route(web::get().to(controller::users::get_single_user))
            .route(web::put().to(controller::users::update_user))
            .route(web::delete().to(controller::users::delete_single_user));

        // let upd_user = users_resource;

        App::new()
            .wrap(middleware::Logger::default())
            .data(AppState { service_container })
            .service(authenticate_service)
            .service(users_services)
            .service(signup_service)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
