use actix_web::{get, HttpResponse, Responder, web};
use serde_json::json;

use crate::{AppState, models::user::User};

#[get("/top/players")]
async fn handler(state: web::Data<AppState>) -> impl Responder {
    let users: Vec<User> = sqlx::query_as("SELECT * FROM users LEFT JOIN accounts ON users.extID = accounts.accountID WHERE stars > 10 ORDER BY stars DESC LIMIT 50")
        .fetch_all(&state.db).await.unwrap();

    let mut result: Vec<serde_json::Value> = vec![];

    for user in users.iter().enumerate() {
        result.push(json!({
            "top_position": user.0 + 1,
            "username": user.1.username,
            "stars": user.1.stars,
            "diamonds": user.1.diamonds,
            "coins": user.1.coins,
            "silver_coins": user.1.user_coins,
            "demons": user.1.demons,
            "creator_points": user.1.creator_points.round().rem_euclid(2f64.powi(32)) as u32 as i32,
            "cube": user.1.acc_icon,
            "ship": user.1.acc_ship,
            "ball": user.1.acc_ball,
            "ufo": user.1.acc_bird,
            "wave": user.1.acc_dart,
            "robot": user.1.acc_robot,
            "spider": user.1.acc_spider,
            "glow": user.1.acc_glow,
            "explosion": user.1.acc_explosion,
            "color1": user.1.color1,
            "color2": user.1.color2,
            "icon": user.1.icon,
            "icon_type": user.1.icon_type,
            "special": user.1.special,
        }));
    }
    
    HttpResponse::Ok().json(json!({
        "success": true,
        "users": result,
    }))
}
