use actix_web::{web, HttpResponse, Result};

use crate::core::database::DbPool;
use crate::core::ExceptionBuilder;
use crate::dtos::client_dtos::{CreateClientRequest, GetClientRequest};
use crate::repositories::client_repository;

pub async fn create_client(
    pool: web::Data<DbPool>,
    info: web::Json<CreateClientRequest>,
) -> Result<HttpResponse> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    match client_repository::create_client(
        &mut conn,
        info.name.clone(),
        info.secret.clone(),
        info.redirect_uri.clone(),
    ) {
        Ok(client) => Ok(HttpResponse::Ok().json(client)),
        Err(err) => Ok(HttpResponse::InternalServerError().json(
            ExceptionBuilder::new_internal_server_error_exception()
                .message(format!("Error: {}", err))
                .build(),
        )),
    }
}

pub async fn find_client_by_id(
    pool: web::Data<DbPool>,
    info: web::Path<GetClientRequest>,
) -> Result<HttpResponse> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    match client_repository::find_client_by_id(&mut conn, info.client_id) {
        Ok(client) => Ok(HttpResponse::Ok().json(client)),
        Err(err) => Ok(HttpResponse::NotFound().json(
            ExceptionBuilder::new_internal_server_error_exception()
                .message(format!("Error: {}", err))
                .build(),
        )),
    }
}
