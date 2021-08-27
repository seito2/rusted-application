use actix_web::{get, web, App, HttpServer, Responder};
use dotenv::dotenv;

mod config;

#[get("/{id}/{name}")]
async fn test(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::load_env().unwrap();

    HttpServer::new(|| App::new().service(index))
        .bind(config.server_address.clone())?
        .run()
        .await
}
