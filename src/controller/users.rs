use crate::models::users::{AuthenticateUser, AuthorizedUser, Info, User};
use crate::AppState;
use actix_web::error::{BlockingError, ErrorBadRequest};
use actix_web::{
    dev, web, App, Error, FromRequest, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_validator::ValidatedJson;
use futures::future::{err, ok, Ready};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

pub async fn signup_user(
    app_data: web::Data<AppState>,
    user: ValidatedJson<User>,
) -> impl Responder {
    let result = web::block(move || {
        app_data.service_container.user.create(
            &user.username,
            &user.password,
            &user.email,
            &user.organization,
            &user.role,
        )
    })
    .await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(BlockingError::Error(user_error)) => HttpResponse::BadRequest().json(user_error),
        Err(BlockingError::Canceled) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_single_user(
    app_data: web::Data<crate::AppState>,
    info: web::Path<Info>,
    authorized_user: Option<AuthorizedUser>,
) -> impl Responder {
    if authorized_user.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let requestor = authorized_user.unwrap();

    if (&requestor.sub != &info.user_id) | (&requestor.role != "Admin") {
        return HttpResponse::Unauthorized().finish();
    }

    let auth_res = web::block(move || app_data.service_container.user.get(&info.user_id)).await;

    match auth_res {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(BlockingError::Error(user_error)) => HttpResponse::BadRequest().json(user_error),
        Err(BlockingError::Canceled) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn authenticate_user(
    app_data: web::Data<crate::AppState>,
    user: ValidatedJson<AuthenticateUser>,
) -> impl Responder {
    let result = web::block(move || {
        app_data
            .service_container
            .user
            .authenticate(&user.username, &user.password)
    })
    .await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(BlockingError::Error(auth_error)) => HttpResponse::BadRequest().json(auth_error),
        Err(BlockingError::Canceled) => HttpResponse::InternalServerError().finish(),
    }
}
