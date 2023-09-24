use actix_web::{post, HttpResponse, Responder, web};
use serde::Deserialize;
use serde_json::json;

use crate::{AppState, models::user::User};

#[derive(Deserialize)]
struct Body {
    username: String,
    password: String,
}

#[post("/users/login")]
async fn handler(body: web::Json<Body>, state: web::Data<AppState>) -> impl Responder {
    let user: User = match sqlx::query_as("SELECT * FROM accounts WHERE username = ? AND isActive = 1")
        .bind(&body.username)
        .fetch_optional(&state.db).await.unwrap()
    {
        Some(user) => user,
        None => {
            return HttpResponse::Unauthorized().json(json!({
                "success": false,
                "error": "Wrong login or password",
                "error_code": "WRONG_CREDENTIALS",
            }));
        },
    };

    if !bcrypt::verify(&body.password, &user.password).unwrap() {
        return HttpResponse::Unauthorized().json(json!({
            "success": false,
            "error": "Wrong login or password",
            "error_code": "WRONG_CREDENTIALS",
        }));
    }

    HttpResponse::Ok().json(json!({
        "success": true,
        "token": &user.token,
    }))
}
