use actix_web::{get, put, Responder};

#[get("/me")]
pub async fn get_profile() -> impl Responder {
    "Get profile"
}

#[put("/me")]
pub async fn update_profile() -> impl Responder {
    "Update profile"
}