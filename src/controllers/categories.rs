use actix_web::{get, post, put, delete, Responder, HttpResponse, web, HttpRequest};
use crate::{db, AppState, utils};
use serde::Deserialize;
use serde_json::json;

#[get("/categories")]
pub async fn  index(state: web::Data<AppState>, req: HttpRequest) ->impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);
    let categories = db::categories::get_all_of_user(&db, user_id).await;
    HttpResponse::Ok().json(categories)
}

#[derive(Deserialize, Debug)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub balance: u64,
}

#[post("/categories")]
pub async fn create(
    state: web::Data<AppState>, 
    req: HttpRequest, 
    data: web::Json<CreateCategoryRequest>) ->impl Responder 
{
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);
    let category = db::categories::create_category(&db, user_id, &data).await.unwrap();
    HttpResponse::Created().json(category)
}

#[get("/categories/{id}")]
pub async fn show(state: web::Data<AppState>, req: HttpRequest, path: web::Path<u64>) ->impl Responder {
    let user_id = utils::get_user_id(&req);
    let db = state.db.lock().await;
    let id = path.into_inner();
    let Some(category) = db::categories::get_category_by_id(&db, id).await else {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Category not found"
        }));
    };

    if category.user_id != user_id {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Unauthorized"
        }));
    }

    HttpResponse::Ok().json(category)
}

#[derive(Deserialize, Debug)]
pub struct UpdateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub balance: u64,
}

#[put("/categories/{id}")]
pub async fn update(
    state: web::Data<AppState>, 
    req: HttpRequest, 
    path: web::Path<u64>, 
    data: web::Json<UpdateCategoryRequest>
) ->impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);
    let id = path.into_inner();
    let Some(category) = db::categories::get_category_by_id(&db, id).await else {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Category not found"
        }));
    };

    if category.user_id != user_id {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Unauthorized"
        }));
    }

    let category = db::categories::update_category_by_id(&db, category.id, &data).await;

    HttpResponse::Ok().json(category)
}

#[delete("/categories/{id}")]
pub async fn destroy(state: web::Data<AppState>, req: HttpRequest, path: web::Path<u64>) ->impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);
    let id = path.into_inner();
    let Some(category) = db::categories::get_category_by_id(&db, id).await else {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Category not found"
        }));
    };
    

    if category.user_id != user_id {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Unauthorized"
        }));
    }

    db::categories::delete_category_by_id(&db, category.id).await;

    HttpResponse::Ok().json(json!({
        "status": "success"
    }))
}

// List transactions by category id
#[get("/category/{id}/transactions")]
pub async fn transactions(state: web::Data<AppState>, req: HttpRequest, path: web::Path<u64>) ->impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);
    let category_id = path.into_inner();

    let Some(category) = db::categories::get_category_by_id(&db, category_id).await else {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Category not found"
        }));
    };
    
    if category.user_id != user_id {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Unauthorized"
        }));
    }

    let transactions = db::transactions::get_all_by_category(&db, category_id).await;
    HttpResponse::Ok().json(transactions)
}
