use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("/health")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust, SQLX, Postgres, and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/checker").service(health_checker_handler);

    conf.service(scope);
}
