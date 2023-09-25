use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::HttpRequest;
use jsonwebtoken::EncodingKey;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;

use crate::models::account::Account;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
}

pub fn generate_jwt(username: String) -> String {
    let start: SystemTime = SystemTime::now();
    let timestamp: usize = start
        .duration_since(UNIX_EPOCH).unwrap()
        .as_secs() as usize;

    let claims: Claims = Claims {
        sub: username,
        iat: timestamp,
        exp: timestamp + 2630000,
    };

    jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &EncodingKey::from_secret(std::env::var("SECRET").unwrap().as_ref())).unwrap()
}

pub async fn authenticate(req: HttpRequest, db: &MySqlPool) -> Option<Account> {
    let token = req.headers().get("Authorization")?.to_str().unwrap().to_string();
    
    sqlx::query_as("SELECT * FROM accounts WHERE token = ? AND isActive = 1")
        .bind(&token)
        .fetch_optional(db).await.unwrap()
}
