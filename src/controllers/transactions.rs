use actix_web::{get, post, put, delete, Responder, HttpResponse, web, HttpRequest};
use crate::{db, AppState, utils};
use serde::Deserialize;
use serde_json::json;
use crate::utils::{is_credit, is_debit};

#[get("/transactions")]
pub async fn index(state: web::Data<AppState>, req: HttpRequest) ->impl Responder {
    
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);
    let transactions = db::transactions::get_all_of_user(&db, user_id).await;
    HttpResponse::Ok().json(transactions)   
}

#[derive(Deserialize, Debug)]
pub struct CreateTransactionRequest {
    pub category_id: u64,
    pub r#type: String,
    pub amount: u64,
    pub memo: Option<String>,
    pub description: Option<String>,
}

#[post("/transactions")]
pub async fn create(
    state: web::Data<AppState>, 
    req: HttpRequest, 
    data: web::Json<CreateTransactionRequest>
) ->impl Responder {
    let db = state.db.lock().await;
    let user = utils::get_authenticated_user(&req, &db).await;

    let Some(category) = db::categories::get_category_by_id(&db,data.category_id).await else {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Category not found"
        }));
    };

    if category.user_id != user.id {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Unauthorized"
        }));
    }

    if data.r#type == "DEBIT" && ( user.balance < data.amount || category.balance < data.amount) {
        return HttpResponse::UnprocessableEntity().json(json!({
            "status": "error",
            "message": "Insufficient balance"
        }));
    }

    let transaction = db::transactions::create_transaction(&db, user.id, &data).await.unwrap();

    let new_balance = if data.r#type == "DEBIT" {
        user.balance - data.amount
    } else {
        user.balance + data.amount
    };

    let new_category_balance = if data.r#type == "DEBIT" {
        category.balance - data.amount
    } else {
        category.balance + data.amount
    };

    db::categories::update_category_balance(&db, category.id, new_category_balance).await;
    db::user::update_user_balance(&db, user.id, new_balance).await;

    HttpResponse::Created().json(transaction)
}

#[get("/transactions/{id}")]
pub async fn show(state: web::Data<AppState>, req: HttpRequest, path: web::Path<u64>) ->impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);
    let id = path.into_inner();
    
    let Some(transaction) = db::transactions::get_transaction_by_id(&db, id).await else {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Transaction not found"
        }));
    };
    
    if transaction.user_id != user_id {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Unauthorized"
        }));
    }
    
    HttpResponse::Ok().json(transaction)
}

#[derive(Deserialize, Debug)]
pub struct UpdateTransactionRequest {
    pub memo: Option<String>,
    pub description: Option<String>,
}

#[put("/transactions/{id}")]
pub async fn update(
        state: web::Data<AppState>, 
        req: HttpRequest, 
        path: web::Path<u64>, 
        data: web::Json<UpdateTransactionRequest>
    ) ->impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);
    let id = path.into_inner();
    let Some(transaction) = db::transactions::get_transaction_by_id(&db, id).await else {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Transaction not found"
        }));
    };

    if transaction.user_id != user_id {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Unauthorized"
        }));
    }
    
    let transaction = db::transactions::update_transaction_by_id(&db, transaction.id, &data).await;

    HttpResponse::Ok().json(transaction)
}

#[delete("/transactions/{id}")]
pub async fn destroy(state: web::Data<AppState>, req: HttpRequest, path: web::Path<u64>) ->impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);
    let id = path.into_inner();
    let user = utils::get_authenticated_user(&req, &db).await;

    let Some(transaction) = db::transactions::get_transaction_by_id(&db, id).await else {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Transaction not found"
        }));
    };
    
    if transaction.user_id != user_id {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Unauthorized"
        }));
    }
    
    let category = db::categories::get_category_by_id(&db, transaction.category_id).await.unwrap();

    if is_credit(&transaction.r#type) 
        && (transaction.amount > category.balance || transaction.amount > category.balance) {
            return HttpResponse::BadRequest().json(json!({
                "status": "error",
                "message": "Insufficient balance"
            }));
    } 

    db::transactions::delete_transaction_by_id(&db, transaction.id).await;

    let user_balance = if is_debit(&transaction.r#type) {
        user.balance + transaction.amount
    } else {
        user.balance - transaction.amount
    };

    db::user::update_user_balance(&db, user.id, user_balance).await;

    let category_balance = if is_debit(&transaction.r#type) {
        category.balance + transaction.amount
    } else {
        category.balance - transaction.amount
    };

    db::categories::update_category_balance(&db, category.id, category_balance).await;
    
    HttpResponse::Ok().json(json!({
        "status": "success"
    }))
}