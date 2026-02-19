use crate::controllers::transactions::{CreateTransactionRequest, UpdateTransactionRequest};
use chrono::NaiveDateTime;
use serde::{Serialize};

#[derive(Serialize)]
pub struct Transaction {
    pub id: u64,
    pub user_id: u64,
    pub category_id: u64,
    pub r#type: String,
    pub amount: u64,
    pub memo: Option<String>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub async fn get_all_of_user(db: &sqlx::MySqlPool, user_id: u64) -> Vec<Transaction> {
    sqlx::query_as!(Transaction, "SELECT * FROM transactions WHERE user_id = ?", user_id)
        .fetch_all(db)
        .await
        .unwrap()
}

pub async fn create_transaction(
    db: &sqlx::MySqlPool,
    user_id: u64,
    data: &CreateTransactionRequest
) -> Option<Transaction> {
    let result = sqlx::query_as!(
        Transaction, 
        "INSERT INTO transactions (user_id, category_id, type, amount, memo, description) VALUES (?, ?, ?, ?, ?, ?)", 
        &user_id, &data.category_id, &data.r#type, &data.amount, &data.memo, &data.description)
        .execute(db)
        .await
        .unwrap();

    get_transaction_by_id(db, result.last_insert_id()).await
}

pub async fn get_transaction_by_id(db: &sqlx::MySqlPool, id: u64) -> Option<Transaction> {
    sqlx::query_as!(Transaction, "SELECT * FROM transactions WHERE id = ?", id)
        .fetch_one(db)
        .await
        .ok()
}

pub async fn update_transaction_by_id(db: &sqlx::MySqlPool, id: u64, data: &UpdateTransactionRequest) -> Transaction {
    sqlx::query_as!(
        Transaction, 
        "UPDATE transactions SET memo = ?, description = ? WHERE id = ?", 
        &data.memo, &data.description, &id
    )
        .execute(db)
        .await
        .ok();

    get_transaction_by_id(db, id).await.unwrap()
}

pub async fn delete_transaction_by_id(db: &sqlx::MySqlPool, id: u64) {
    sqlx::query_as!(
        Transaction, 
        "DELETE FROM transactions WHERE id = ?", &id)
        .execute(db)
        .await
        .unwrap();
}


pub async fn get_all_by_category(db: &sqlx::MySqlPool, category_id: u64) -> Vec<Transaction> {
    sqlx::query_as!(Transaction, "SELECT * FROM transactions WHERE category_id = ?", category_id)
        .fetch_all(db)
        .await
        .unwrap()
}