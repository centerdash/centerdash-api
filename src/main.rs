use actix_web::{HttpServer, App, web};
use sqlx::mysql::MySqlPoolOptions;
use dotenv::dotenv;

mod routes;
mod models;
mod helpers;

struct AppState {
    db: sqlx::MySqlPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::var("DATABASE_URL").expect("DATABASE_URL not found in your .env");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(std::env::var("DATABASE_URL").unwrap().as_str()).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(routes::login::handler)
            .service(routes::register::handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
