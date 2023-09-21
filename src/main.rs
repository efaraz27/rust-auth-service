mod core;
mod dtos;
mod models;
mod repositories;
mod routes;
mod schema;
mod services;
mod middlewares;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use core::database::create_db_pool;
use routes::{client_routes, user_routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = create_db_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(user_routes::configure)
            .configure(client_routes::configure)
            .route("/", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("API is running")
}
