use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    #[sqlx(rename = "accountID")]
    pub account_id: i32,
    #[sqlx(rename = "userName")]
    pub username: String,
    pub password: String,
    pub token: String,
}
