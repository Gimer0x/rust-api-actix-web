use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::controllers::categories::{CreateCategoryRequest, UpdateCategoryRequest};

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub id: u64,
    pub user_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub balance: u64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub async fn get_all_of_user(db: &sqlx::MySqlPool, user_id: u64) -> Vec<Category> {
    sqlx::query_as!(Category, "SELECT * FROM categories WHERE user_id = ?", user_id)
        .fetch_all(db)
        .await
        .unwrap()
}

pub async fn get_category_by_id(db: &sqlx::MySqlPool, id: u64) -> Option<Category> {
    sqlx::query_as!(Category, "SELECT * FROM categories WHERE id = ?", id)
        .fetch_one(db)
        .await
        .ok()
}

pub async fn create_category(
    db: &sqlx::MySqlPool, user_id: u64, data: &CreateCategoryRequest
) -> Option<Category> {
    let result = sqlx::query_as!(
        Category, 
        "INSERT INTO categories (user_id, name, description, balance) VALUES (?, ?, ?, ?)", 
        &user_id, &data.name, &data.description, &data.balance)
        .execute(db)
        .await
        .unwrap();
    
    get_category_by_id(db, result.last_insert_id()).await
}

pub async fn update_category_by_id(db: &sqlx::MySqlPool, id: u64, data: &UpdateCategoryRequest) -> Option<Category> {
    sqlx::query_as!(
        Category, 
        "UPDATE categories SET name = ?, description = ?, balance = ? WHERE id = ?", 
        &data.name, &data.description, &data.balance, &id)
        .execute(db)
        .await
        .unwrap();

    get_category_by_id(db, id).await
}

pub async fn delete_category_by_id(db: &sqlx::MySqlPool, id: u64){
    let _ = sqlx::query_as!(
        Category, 
        "DELETE FROM categories WHERE id = ?", &id)
        .execute(db)
        .await;
}

pub async fn update_category_balance(db: &sqlx::MySqlPool, id: u64, balance: u64) {
    sqlx::query!(
        "UPDATE categories SET balance = balance + ? WHERE id = ?",
        &balance, &id
    )
    .execute(db)
    .await
    .unwrap();
}