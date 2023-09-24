use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::EncodingKey;
use serde::{Serialize, Deserialize};

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
