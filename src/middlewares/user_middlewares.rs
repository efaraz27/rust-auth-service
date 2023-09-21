use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse as DevServiceResponse},
    Error as ActixError, HttpMessage,
};
use futures::future::Ready;

use crate::core::helpers::jwt::get_authenticated_user_from_request;
use crate::models::user::UserRole;

async fn admin_middleware(
    req: ServiceRequest,
    srv: &dyn Service<
        ServiceRequest,
        Response = DevServiceResponse,
        Error = ActixError,
        Future = Ready<Result<DevServiceResponse, ActixError>>,
    >,
) -> Result<DevServiceResponse, actix_web::Error> {
    if let Ok(Some(user)) = get_authenticated_user_from_request(&req).await {
        if user.role == UserRole::Admin.to_string() {
            return Ok(srv.call(req).await?);
        }
    }
    Err(actix_web::error::ErrorUnauthorized("Only admins allowed"))
}

async fn set_user_in_request_middleware(
    req: ServiceRequest,
    srv: &dyn Service<
        ServiceRequest,
        Response = DevServiceResponse,
        Error = ActixError,
        Future = Ready<Result<DevServiceResponse, ActixError>>,
    >,
) -> Result<DevServiceResponse, actix_web::Error> {
    if let Ok(Some(user)) = get_authenticated_user_from_request(&req).await {
        req.extensions_mut().insert(user);
        return Ok(srv.call(req).await?);
    }
    Err(actix_web::error::ErrorUnauthorized("Not authorized"))
}
