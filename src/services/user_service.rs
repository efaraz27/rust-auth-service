use actix_web::{web, HttpResponse, Result};
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::core::database::DbPool;
use crate::core::helpers::jwt::generate_token;
use crate::core::ExceptionBuilder;
use crate::dtos::user_dtos::{LoginRequest, LoginResponse, RegisterRequest};
use crate::models::User;
use crate::repositories::user_repository;

const ACCESS_TOKEN_EXP: usize = 900; // Duration in seconds
const REFRESH_TOKEN_EXP: usize = 3600; // Duration in seconds

pub async fn register(
    pool: web::Data<DbPool>,
    info: web::Json<RegisterRequest>,
) -> Result<HttpResponse> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    let hashed_password = match hash(&info.password, DEFAULT_COST) {
        Ok(hp) => hp,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(
                ExceptionBuilder::new_internal_server_error_exception()
                    .message("Something went wrong".to_string())
                    .build(),
            ))
        }
    };

    match user_repository::create_user(&mut conn, info.email.clone(), hashed_password) {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => Ok(HttpResponse::InternalServerError().json(
            ExceptionBuilder::new_internal_server_error_exception()
                .message(format!("Error: {}", err))
                .build(),
        )),
    }
}

pub async fn login(pool: web::Data<DbPool>, info: web::Json<LoginRequest>) -> Result<HttpResponse> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    match user_repository::find_user_by_email(&mut conn, &info.email) {
        Ok(user) => {
            let is_valid = verify(&info.password, &user.password_hash).unwrap_or(false);

            if is_valid {
                let access_token = generate_token(&user.email, ACCESS_TOKEN_EXP)?;
                let refresh_token = generate_token(&user.email, REFRESH_TOKEN_EXP)?;

                let response = LoginResponse {
                    access_token,
                    refresh_token,
                };

                Ok(HttpResponse::Ok().json(response))
            } else {
                Ok(HttpResponse::Unauthorized().json(
                    ExceptionBuilder::new_unauthorized_exception()
                        .message("Invalid email or password".to_string())
                        .build(),
                ))
            }
        }
        Err(err) => Ok(HttpResponse::InternalServerError().json(
            ExceptionBuilder::new_internal_server_error_exception()
                .message(format!("Error: {}", err))
                .build(),
        )),
    }
}

pub async fn find_user_by_email(pool: web::Data<DbPool>, user_email: &String) -> Result<User> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    match user_repository::find_user_by_email(&mut conn, user_email) {
        Ok(user) => Ok(user),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(format!(
            "Error: {}",
            err
        ))),
    }
}
