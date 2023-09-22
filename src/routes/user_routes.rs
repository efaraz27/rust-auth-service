use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::core::database::DbPool;
use crate::dtos::user_dtos::{LoginRequest, LoginResponse, RegisterRequest};
use crate::services::user_service;

#[post("/register")]
async fn register(pool: web::Data<DbPool>, info: web::Json<RegisterRequest>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    let response = user_service::register(&mut conn, &info.email, &info.password).await;

    match response {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => err.to_http_response(),
    }
}

#[post("/login")]
async fn login(pool: web::Data<DbPool>, info: web::Json<LoginRequest>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    let response = user_service::login(&mut conn, &info.email, &info.password).await;

    match response {
        Ok(login_response) => HttpResponse::Ok().json(login_response),
        Err(err) => err.to_http_response(),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
}
