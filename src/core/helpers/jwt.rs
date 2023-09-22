use actix_web::dev::ServiceRequest;
use actix_web::web;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::core::database::DbPool;
use crate::core::exceptions::{Exception, ExceptionBuilder};
use crate::models::User;
use crate::services::user_service;

const SECRET_KEY: &[u8] = b"secret_key";

/// JWT claims.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

/// Generate a JWT token for the given email and expiration time.
pub fn generate_token(email: &str, expiration: usize) -> Result<String, Exception> {
    let claims = Claims {
        sub: email.to_string(),
        exp: expiration,
    };

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    ) {
        Ok(token) => Ok(token),
        Err(_) => Err(ExceptionBuilder::new_internal_server_error_exception()
            .message("Failed to generate token".to_string())
            .build()),
    }
}

/// Decode a JWT token and return its claims.
pub fn decode_token(token: &str) -> Result<Claims, Exception> {
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    ) {
        Ok(token_data) => Ok(token_data.claims),
        Err(_) => Err(ExceptionBuilder::new_forbidden_exception()
            .message("Failed to decode token".to_string())
            .build()),
    }
}

/// Get the authenticated user from the given request.
pub async fn get_authenticated_user_from_request(
    req: &ServiceRequest,
) -> Result<Option<User>, Exception> {
    let auth_header = req.headers().get("Authorization");

    let access_token = match auth_header {
        Some(token) => token.to_str().unwrap().replace("Bearer ", ""),
        None => return Ok(None),
    };

    let claims = decode_token(&access_token)?;

    let pool = match req.app_data::<web::Data<DbPool>>() {
        Some(pool) => pool.clone(),
        None => {
            return Err(ExceptionBuilder::new_internal_server_error_exception()
                .message("Something went wrong".to_string())
                .build())
        }
    };

    let user = match user_service::find_user_by_email(pool, &claims.sub).await {
        Ok(user) => user,
        Err(err) => return Err(err),
    };

    Ok(Some(user))
}
