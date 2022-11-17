use actix_web::{web, HttpResponse};
use serde_json::json;

use super::model::{UpdateUser, User};
use crate::errors::ApiError;

pub async fn get_users() -> Result<HttpResponse, ApiError> {
    let users = User::get()?;
    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_user_by_id(id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let user = User::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn update_user(
    user: web::Json<UpdateUser>,
    id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let updated_user = User::update(user.into_inner(), id.into_inner())?;
    Ok(HttpResponse::Ok().json(updated_user))
}

pub async fn delete_user(id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let count = User::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": count })))
}
