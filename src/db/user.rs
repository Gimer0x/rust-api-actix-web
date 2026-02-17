use crate::controllers::auth::SignUpRequest;
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