use crate::models::users::{AuthenticateUser, AuthorizedUser, Info, UpdateUser, User};
use crate::AppState;
use actix_web::error::BlockingError;
use actix_web::{web, HttpResponse, Responder};
use actix_web_validator::ValidatedJson;

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

pub async fn update_user(
    app_data: web::Data<AppState>,
    info: web::Path<Info>,
    updates: ValidatedJson<UpdateUser>,
    authorized_user: Option<AuthorizedUser>,
) -> impl Responder {
    let requestor = authorized_user.unwrap();

    if (&requestor.sub != &info.user_id) | (&requestor.role != "Admin") {
        // dbg!("requestor.sub: {}", &requestor.sub);
        // dbg!("info.user_id: {}", &info.user_id);
        // dbg!("User did not pass basic logic");
        return HttpResponse::Unauthorized().finish();
    }

    let result = web::block(move || {
        let d = updates.into_inner();
        app_data.service_container.user.update(&info.user_id, d)
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
        // dbg!("requestor.sub: {}", &requestor.sub);
        // dbg!("info.user_id: {}", &info.user_id);
        // dbg!("User did not pass basic logic");
        return HttpResponse::Unauthorized().finish();
    }

    let auth_res = web::block(move || app_data.service_container.user.get(&info.user_id)).await;

    match auth_res {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(BlockingError::Error(user_error)) => HttpResponse::BadRequest().json(user_error),
        Err(BlockingError::Canceled) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_single_user(
    app_data: web::Data<crate::AppState>,
    info: web::Path<Info>,
    authorized_user: Option<AuthorizedUser>,
) -> impl Responder {
    if authorized_user.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let requestor = authorized_user.unwrap();

    if (&requestor.sub != &info.user_id) | (&requestor.role != "Admin") {
        // dbg!("requestor.sub: {}", &requestor.sub);
        // dbg!("info.user_id: {}", &info.user_id);
        // dbg!("User did not pass basic logic");
        return HttpResponse::Unauthorized().finish();
    }

    let delete_res =
        web::block(move || app_data.service_container.user.delete(&info.user_id)).await;

    match delete_res {
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
