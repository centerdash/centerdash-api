use actix_web::{HttpServer, App, web};
use sqlx::mysql::MySqlPoolOptions;

mod routes;
mod models;

struct AppState {
    db: sqlx::MySqlPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://postgres:password@localhost/test").await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(routes::login::handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
