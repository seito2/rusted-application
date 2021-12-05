use actix_web::HttpResponse;

pub fn ping() -> HttpResponse {
    HttpResponse::Ok().json("pong")
}
