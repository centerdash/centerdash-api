use actix_web::{post, Responder, HttpResponse, web};
use serde::Deserialize;
use serde_json::json;

use crate::{models::account::Account, helpers::{token::generate_jwt, timestamp::get_timestamp}, AppState};

#[derive(Deserialize)]
struct Body {
    username: String,
    password: String,
    captcha: String,
}

#[derive(Deserialize)]
struct CaptchaResult {
    success: bool,
}

#[post("/users/register")]
async fn handler(body: web::Json<Body>, state: web::Data<AppState>) -> impl Responder {
    if body.username.chars().count() > 20 {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "error": "Username can't be more than 20 characters long",
            "error_code": "BAD_USERNAME_LENGTH",
        }));
    }

    if !body.username.chars().all(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9')) {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "error": "Username must contain only English alphanumeric characters",
            "error_code": "BAD_USERNAME_CHARS",
        }));
    }

    let hcaptcha_secret: String;
    let hcaptcha_response: String;

    if std::env::var("HCAPTCHA_SECRET").unwrap() == String::new() {
        hcaptcha_secret = "0x0000000000000000000000000000000000000000".to_string();
        hcaptcha_response = "10000000-aaaa-bbbb-cccc-000000000001".to_string();
    } else {
        hcaptcha_secret = std::env::var("HCAPTCHA_SECRET").unwrap();
        hcaptcha_response = body.captcha.clone();
    }

    let captcha_body = [("response", hcaptcha_response), ("secret", hcaptcha_secret)];

    let client: reqwest::Client = reqwest::Client::new();
    let captcha_result_plain: String = client.post("https://hcaptcha.com/siteverify")
        .form(&captcha_body)
        .send()
        .await.unwrap()
        .text()
        .await.unwrap();

    let captcha_result: CaptchaResult = serde_json::from_str(&captcha_result_plain).unwrap();
    if !captcha_result.success {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "error": "Failed to validate captcha",
            "error_code": "BAD_CAPTCHA",
        }));
    }

    match sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE username = ?")
        .bind(&body.username)
        .fetch_optional(&state.db).await.unwrap()
    {
        Some(_) => {
            return HttpResponse::BadRequest().json(json!({
                "success": false,
                "error": "Username already taken",
                "error_code": "USERNAME_TAKEN",
            }));
        },
        None => {},
    };

    let token: String = generate_jwt(body.username.clone());
    let password_hashed: String = bcrypt::hash(&body.password.clone(), bcrypt::DEFAULT_COST).unwrap();
    
    sqlx::query("INSERT INTO accounts (username, password, email, registerDate, isActive, token) VALUES (?, ?, 'no@email.com', ?, 1, ?)")
        .bind(&body.username)
        .bind(&password_hashed)
        .bind(get_timestamp())
        .bind(&token)
        .execute(&state.db).await.unwrap();

    HttpResponse::Ok().json(json!({
        "success": true,
        "token": token,
    }))
}
