use actix_web::dev::ServiceRequest;
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::web;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::core::database::DbPool;
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
pub fn generate_token(email: &str, expiration: usize) -> Result<String, actix_web::Error> {
    let claims = Claims {
        sub: email.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )
    .map_err(|err| InternalError::new(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR).into())
}

/// Decode a JWT token and return its claims.
pub fn decode_token(token: &str) -> Result<Claims, actix_web::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|err| InternalError::new(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR).into())
}

/// Get the authenticated user from the given request.
pub async fn get_authenticated_user_from_request(
    req: &ServiceRequest,
) -> Result<Option<User>, actix_web::Error> {
    let headers = req.headers();
    if let Some(auth_header) = headers.get("Authorization") {
        let auth_header = auth_header.to_str().map_err(|err| {
            InternalError::new(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
        })?;
        let token = auth_header.replace("Bearer ", "");
        let claims = decode_token(&token)?;
        let pool = req
            .app_data::<web::Data<DbPool>>()
            .ok_or_else(|| {
                InternalError::new(
                    "Failed to get DB pool from request",
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            })?
            .clone();
        let user = user_service::find_user_by_email(pool, &claims.sub)
            .await
            .map_err(|err| {
                InternalError::new(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
            })?;

        Ok(Some(user))
    } else {
        Ok(None)
    }
}
