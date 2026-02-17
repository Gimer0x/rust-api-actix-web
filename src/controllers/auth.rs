use actix_web::{post, Responder, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::MutexGuard;
use sqlx::{Pool, MySql};

use crate::{db, AppState};

#[derive(Deserialize, Debug)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[post("/auth/sign-up")]
pub async fn sign_up(state: web::Data<AppState>, data: web::Json<SignUpRequest>) -> impl Responder {
    let db: MutexGuard<'_, Pool<MySql>> = state.db.lock().await;
    if db::user::has_user_with_email_exists(&db, &data.email).await {
        return HttpResponse::UnprocessableEntity().json(json!({
            "status": "error",
            "message": "Email already exists"
        }));
    }

    db::user::create_user(&db, data.into_inner()).await;

    HttpResponse::Created().json(json!({
        "status": "success",
        "message": "User created successfully"
    }))
}

#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    "Sign in"
}