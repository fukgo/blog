use crate::models::user::User;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};
use std::sync::Mutex;

#[derive(Debug, Deserialize)]
pub struct ArticleQueryParams {
    pub title: Option<String>,
    pub user_id: Option<i32>,
    pub tag_id: Option<i32>,
    pub page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct PageParams {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}
