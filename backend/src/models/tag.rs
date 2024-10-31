use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Tag {
    pub id: i64,
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct BlogTag {
    pub id: i64,
    pub blog_id: i64,
    pub tag_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagCreate {
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogTagCreate {
    pub blog_id: i64,
    pub tag_id: i64,
}
