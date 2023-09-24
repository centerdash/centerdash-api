use actix_web::{HttpServer, App, web, middleware::Logger, http};
use sqlx::mysql::MySqlPoolOptions;
use actix_cors::Cors;
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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    std::env::var("DATABASE_URL").expect("DATABASE_URL not found in your .env");
    std::env::var("HCAPTCHA_SECRET").expect("HCAPTCHA_SECRET not found in your .env");
    std::env::var("SECRET").expect("SECRET not found in your .env");

    let port = std::env::var("PORT")
        .expect("PORT not found in your .env")
        .parse::<u16>().unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(std::env::var("DATABASE_URL").unwrap().as_str()).await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://www.centerdash.ru")
            .allowed_origin("https://centerdash.ru")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(routes::login::handler)
            .service(routes::register::handler)
            .wrap(Logger::default())
            .wrap(cors)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
