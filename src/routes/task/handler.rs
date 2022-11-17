use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde_json::json;

use super::model::{NewTask, Task, UpdateTask};
use crate::errors::ApiError;

pub async fn create_task(
    new_task: web::Json<NewTask>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let user_id: Option<String> = session.get("user_id")?;
    if let Some(_) = user_id {
        let task = Task::create(new_task.into_inner(), user_id.unwrap())?;
        Ok(HttpResponse::Ok().json(task))
    } else {
        Err(ApiError::new(401, "Unauthorized".to_string()))
    }
}

pub async fn get_tasks(session: Session) -> Result<HttpResponse, ApiError> {
    let user_id: Option<String> = session.get("user_id")?;
    if let Some(_) = user_id {
        let tasks = Task::get(user_id.unwrap())?;
        Ok(HttpResponse::Ok().json(tasks))
    } else {
        Err(ApiError::new(401, "Unauthorized".to_string()))
    }
}

pub async fn get_task_by_id(id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    let task = Task::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(task))
}

pub async fn update_task(
    task: web::Json<UpdateTask>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let user_id: Option<String> = session.get("user_id")?;
    if let Some(_) = user_id {
        let task_user_id = Task::find(task.id.clone())?.user_id;
        if task_user_id != user_id.unwrap() {
            return Err(ApiError::new(401, "Unauthorized".to_string()));
        }
        let task = Task::update(task.into_inner())?;
        Ok(HttpResponse::Ok().json(task))
    } else {
        Err(ApiError::new(401, "Unauthorized".to_string()))
    }
}

pub async fn delete_task(
    id: web::Path<String>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let user_id: Option<String> = session.get("user_id")?;
    if let Some(_) = user_id {
        let task_user_id = Task::find(id.clone())?.user_id;
        if task_user_id != user_id.unwrap() {
            return Err(ApiError::new(401, "Unauthorized".to_string()));
        } else {
            let count = Task::delete(id.into_inner())?;
            Ok(HttpResponse::Ok().json(json!({ "deleted": count })))
        }
    } else {
        return Err(ApiError::new(401, "Unauthorized".to_string()));
    }
}
