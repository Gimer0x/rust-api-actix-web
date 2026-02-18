use crate::controllers::me::UpdateProfileRequest;
use crate::controllers::auth::SignUpRequest;
use serde::Serialize;
use sqlx::types::chrono;
use bcrypt::{DEFAULT_COST, hash};

pub async fn has_user_with_email_exists(db: &sqlx::MySqlPool, email: &str) -> bool {
    sqlx::query!("SELECT * FROM users WHERE email = ?", email)
        .fetch_optional(db)
        .await
        .unwrap()
        .is_some()
}

pub async fn create_user(db: &sqlx::MySqlPool, user: SignUpRequest) -> bool {
    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();
    sqlx::query!(
        "INSERT INTO users (email, firstname, lastname, password) VALUES (?, ?, ?, ?)",
        &user.email, &user.firstname, &user.lastname, &hashed_password
    )
    .execute(db)
    .await
    .is_ok()
}

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub balance: u64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn get_user_by_email(db: &sqlx::MySqlPool, email: &str) -> Option<User> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email)
        .fetch_optional(db)
        .await
        .unwrap()
}

pub async fn get_user_by_id(db: &sqlx::MySqlPool, id: u64) -> Option<User> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
        .fetch_optional(db)
        .await
        .unwrap()
}

pub async fn update_user_by_id(db: &sqlx::MySqlPool, id: u64, user: &UpdateProfileRequest) {
    sqlx::query!(
        "UPDATE users SET firstname = ?, lastname = ? WHERE id = ?",
        &user.firstname, &user.lastname, id
    )
    .execute(db)
    .await
    .unwrap();
}