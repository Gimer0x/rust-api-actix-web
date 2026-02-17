use actix_web::{post, Responder, web};
use serde::Deserialize;
use tokio::sync::MutexGuard;
use sqlx::{Pool, MySql};

use crate::{db, AppState};

#[derive(Deserialize, Debug)]
struct SignUpRequest {
    email: String,
    password: String,
    firstname: String,
    lastname: String,
}

#[post("/auth/sign-up")]
pub async fn sign_up(state: web::Data<AppState>, data: web::Json<SignUpRequest>) -> impl Responder {
    let db: MutexGuard<'_, Pool<MySql>> = state.db.lock().await;
    if db::user::has_user_with_email_exists(&db, &data.email).await {
        return "User already exists".to_string();
    }

    format!("Sign up: {:?}", data)
}

#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    "Sign in"
}