use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/client")
            .route("", web::post().to(crate::services::client_service::create_client))
            .route("/{client_id}", web::get().to(crate::services::client_service::find_client_by_id))
    );
}