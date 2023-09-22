use actix_web::{web, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;

use crate::core::database::DbPool;
use crate::core::exceptions::{Exception, ExceptionBuilder};
use crate::core::helpers::jwt::generate_token;
use crate::dtos::user_dtos::LoginResponse;
use crate::models::User;
use crate::repositories::user_repository;

const ACCESS_TOKEN_EXP: usize = 900; // Duration in seconds
const REFRESH_TOKEN_EXP: usize = 3600; // Duration in seconds

pub async fn register(
    conn: &mut PgConnection,
    user_email: &String,
    user_password: &String,
) -> Result<User, Exception> {
    // Check if user already exists
    match user_repository::find_user_by_email(conn, user_email) {
        Ok(_) => {
            return Err(ExceptionBuilder::new_bad_request_exception()
                .message("User already exists".to_string())
                .build())
        }
        Err(_) => {}
    }

    // Hash password
    let password_hash = match hash(user_password.clone(), DEFAULT_COST) {
        Ok(hp) => hp.to_string(),
        Err(_) => {
            return Err(ExceptionBuilder::new_internal_server_error_exception()
                .message("Something went wrong".to_string())
                .build())
        }
    };

    // Create user
    let user = match user_repository::create_user(conn, user_email, &password_hash) {
        Ok(user) => user,
        Err(err) => {
            return Err(ExceptionBuilder::new_internal_server_error_exception()
                .message(format!("Error: {}", err))
                .build())
        }
    };

    Ok(user)
}

pub async fn login(
    conn: &mut PgConnection,
    user_email: &String,
    user_password: &String,
) -> Result<LoginResponse, Exception> {
    // Find user by email
    let user = match user_repository::find_user_by_email(conn, user_email) {
        Ok(user) => user,
        Err(err) => {
            // check if the error is not found
            if err == diesel::result::Error::NotFound {
                return Err(ExceptionBuilder::new_unauthorized_exception()
                    .message("Invalid email or password".to_string())
                    .build());
            } else {
                return Err(ExceptionBuilder::new_internal_server_error_exception()
                    .message(format!("Error: {}", err))
                    .build());
            }
        }
    };

    // Verify password
    let is_valid = verify(user_password, &user.password_hash).unwrap_or(false);

    if !is_valid {
        return Err(ExceptionBuilder::new_unauthorized_exception()
            .message("Invalid email or password".to_string())
            .build());
    }

    // Generate tokens
    let access_token = match generate_token(&user.email, ACCESS_TOKEN_EXP) {
        Ok(token) => token,
        Err(_) => {
            return Err(ExceptionBuilder::new_internal_server_error_exception()
                .message("Something went wrong".to_string())
                .build());
        }
    };

    let refresh_token = match generate_token(&user.email, REFRESH_TOKEN_EXP) {
        Ok(token) => token,
        Err(_) => {
            return Err(ExceptionBuilder::new_internal_server_error_exception()
                .message("Something went wrong".to_string())
                .build());
        }
    };

    let response = LoginResponse {
        access_token,
        refresh_token,
    };

    Ok(response)
}

pub async fn find_user_by_email(
    pool: web::Data<DbPool>,
    user_email: &String,
) -> Result<User, Exception> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    match user_repository::find_user_by_email(&mut conn, user_email) {
        Ok(user) => Ok(user),
        Err(err) => {
            // check if the error is not found
            if err == diesel::result::Error::NotFound {
                return Err(ExceptionBuilder::new_not_found_exception()
                    .message("User not found".to_string())
                    .build());
            } else {
                return Err(ExceptionBuilder::new_internal_server_error_exception()
                    .message("Something went wrong".to_string())
                    .build());
            }
        }
    }
}
