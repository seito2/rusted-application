#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};

mod entity;
mod handler;
pub mod models;
mod modules;
pub mod schema;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default() // <- Construct CORS middleware builder
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .route("/ping", web::post().to(handler::other::ping))
            .route(
                "/account/sign-up",
                web::post().to(handler::account::sign_up),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
