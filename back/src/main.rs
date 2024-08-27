use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[get("/")]
async fn index(_req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("Get index");

    Ok(HttpResponse::Ok().body("Welcome!"))
}

pub struct AppState {
    db: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://omica:omica@localhost:5432/ehr")
        .await
    {
        Ok(pool) => pool,
        Err(err) => panic!("Error connecting to database: {}", err),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
