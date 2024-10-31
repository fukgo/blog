use crate::models::user::User;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};
use std::sync::Mutex;

use super::user::UserInfo;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub digest: String,
    pub user_detail_id: i64,
    pub feature: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleDisplay {
    pub id: i64,
    pub title: String,
    pub digest: String,
    pub feature: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub tags: Vec<String>,
    pub author: UserInfo,
    pub total_page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleCreate {
    pub title: String,
    pub content: String,
    pub digest: String,
    pub user_detail_id: i64,
    pub feature: bool,
    pub tags_id: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleUpdate {
    pub title: Option<String>,
    pub content: Option<String>,
    pub digest: Option<String>,
    pub feature: Option<bool>,
    pub tags_id: Option<Vec<i64>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct ArticleTitle {
    pub id: i64,
    pub title: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct ArticleTags {
    pub id: i64,
    pub tag: String,
}
