use actix_web::{post, HttpResponse, Responder, web};

use crate::AppState;

struct Body {
    username: String,
}

#[post("/users/login")]
pub async fn handler(body: web::Json<Body>, state: web::Data<AppState>) -> impl Responder {
    let user = sqlx::query_as("SELECT * FROM accounts WHERE username = ?")
        .bind(&body.username);

    HttpResponse::Ok().body("Hello world!")
}