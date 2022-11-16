use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::errors::ApiError;
use crate::routes::user::model::{NewUser, User};

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

pub async fn register(user: web::Json<NewUser>) -> Result<HttpResponse, ApiError> {
    let new_user = User::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(new_user))
}

pub async fn login(
    credentials: web::Json<AuthData>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let credentials = credentials.into_inner();

    let user = User::find_by_email(credentials.email).map_err(|e| match e.status_code {
        404 => ApiError::new(401, "Credentials no valid!".to_string()),
        _ => e,
    })?;

    let is_valid = user.verify_password(&credentials.password.as_bytes())?;

    if is_valid {
        session.insert("user_id", &user.id)?;
        session.renew();
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ApiError::new(401, "Credentials no valid!".to_string()))
    }
}

pub async fn logout(session: Session) -> Result<HttpResponse, ApiError> {
    let id: Option<String> = session.get("user_id")?;

    if let Some(_) = id {
        session.purge();
        Ok(HttpResponse::Ok().json(json!({ "message": "Logout success!" })))
    } else {
        Err(ApiError::new(401, "Unauthorized".to_string()))
    }
}

pub async fn get_me(session: Session) -> Result<HttpResponse, ApiError> {
    let id: Option<String> = session.get("user_id")?;

    if let Some(id) = id {
        let user = User::find(id)?;
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ApiError::new(401, "Unauthorized".to_string()))
    }
}
