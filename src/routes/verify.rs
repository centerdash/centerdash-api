use actix_web::{post, HttpResponse, Responder, web, HttpRequest};
use serde_json::json;

use crate::{AppState, helpers::token::authenticate};

#[post("/users/verify")]
async fn handler(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    let user = match authenticate(req, &state.db).await {
		Some(user) => user,
		None => {
			return HttpResponse::Unauthorized().json(json!({
                "success": false,
                "error": "Wrong token",
                "error_code": "WRONG_TOKEN",
            }));
		}
	};

    HttpResponse::Ok().json(json!({
        "success": true,
        "username": &user.username,
    }))
}
