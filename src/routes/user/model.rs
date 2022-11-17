use argon2::Config;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::{delete, insert_into, update, RunQueryDsl};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db;
use crate::errors::ApiError;
use crate::schema::users;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct Users(pub Vec<User>);

#[derive(Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn get() -> Result<Users, ApiError> {
        let mut connection = db::connection()?;
        let result = users::table.load::<User>(&mut connection)?;
        Ok(Users(result))
    }

    pub fn create(data: NewUser) -> Result<User, ApiError> {
        let mut connection = db::connection()?;

        let mut new_user = User::from(data);
        new_user.hash_password()?;
        insert_into(users::table)
            .values(&new_user)
            .execute(&mut connection)?;

        let result = users::table.find(new_user.id).first(&mut connection)?;
        Ok(result)
    }

    pub fn find(id: String) -> Result<User, ApiError> {
        let mut connection = db::connection()?;
        let result = users::table.find(id).first(&mut connection)?;
        Ok(result)
    }

    pub fn update(mut data: UpdateUser, id: String) -> Result<User, ApiError> {
        let mut connection = db::connection()?;

        data.updated_at = Some(Utc::now().naive_utc());

        if let Some(password) = data.password {
            data.password = Some(User::hash_password_str(&password)?);
        }

        update(users::table.find(id.clone()))
            .set(&data)
            .execute(&mut connection)?;

        let result = users::table.find(id).first(&mut connection)?;

        Ok(result)
    }

    pub fn delete(id: String) -> Result<usize, ApiError> {
        let mut connection = db::connection()?;
        let count = delete(users::table.find(id)).execute(&mut connection)?;
        Ok(count)
    }

    pub fn find_by_email(email: String) -> Result<User, ApiError> {
        let mut connection = db::connection()?;
        let user = users::table
            .filter(users::email.eq(email))
            .first(&mut connection)?;
        Ok(user)
    }

    pub fn hash_password(&mut self) -> Result<(), ApiError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
            .map_err(|e| ApiError::new(500, format!("Failed to hash password: {}", e)))?;
        Ok(())
    }

    pub fn hash_password_str(password: &str) -> Result<String, ApiError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config)
            .map_err(|e| ApiError::new(500, format!("Failed to hash password: {}", e)))?;
        Ok(hash)
    }

    pub fn verify_password(&self, password: &[u8]) -> Result<bool, ApiError> {
        argon2::verify_encoded(&self.password, password)
            .map_err(|e| ApiError::new(500, format!("Failed to verify password: {}", e)))
    }
}

impl From<NewUser> for User {
    fn from(user: NewUser) -> Self {
        User {
            id: Uuid::new_v4().to_string(),
            username: user.username,
            email: user.email,
            password: user.password,
            firstname: None,
            lastname: None,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}
