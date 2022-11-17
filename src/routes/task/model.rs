use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::{delete, insert_into, update, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db;
use crate::errors::ApiError;
use crate::schema::tasks;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = tasks)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub user_id: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct Tasks(pub Vec<Task>);

#[derive(Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = tasks)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Task {
    pub fn get(user_id: String) -> Result<Tasks, ApiError> {
        let mut connection = db::connection()?;
        let result = tasks::table
            .filter(tasks::user_id.eq(user_id))
            .load::<Task>(&mut connection)?;
        Ok(Tasks(result))
    }

    pub fn create(data: NewTask, user_id: String) -> Result<Task, ApiError> {
        let mut connection = db::connection()?;
        let mut new_task = Task::from(data);
        new_task.user_id = user_id;
        insert_into(tasks::table)
            .values(&new_task)
            .execute(&mut connection)?;
        let result = tasks::table.find(new_task.id).first(&mut connection)?;
        Ok(result)
    }

    pub fn find(id: String) -> Result<Task, ApiError> {
        let mut connection = db::connection()?;
        let result = tasks::table.find(id).first(&mut connection)?;
        Ok(result)
    }

    pub fn update(mut data: UpdateTask, id: String) -> Result<Task, ApiError> {
        let mut connection = db::connection()?;
        data.updated_at = Some(Utc::now().naive_utc());
        update(tasks::table.find(id.clone()))
            .set(&data)
            .execute(&mut connection)?;
        let result = tasks::table.find(id).first(&mut connection)?;
        Ok(result)
    }

    pub fn delete(id: String) -> Result<usize, ApiError> {
        let mut connection = db::connection()?;
        let result = delete(tasks::table.find(id)).execute(&mut connection)?;
        Ok(result)
    }
}

impl From<NewTask> for Task {
    fn from(data: NewTask) -> Self {
        Task {
            id: Uuid::new_v4().to_string(),
            title: data.title,
            description: data.description,
            user_id: "".to_string(),
            completed: false,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}
