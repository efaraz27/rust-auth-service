use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/register", web::post().to(crate::services::user_service::register))
            .route("/login", web::post().to(crate::services::user_service::login))
    );
}