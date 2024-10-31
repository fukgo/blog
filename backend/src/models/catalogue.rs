use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct BlogCatalogue {
    pub id: i32,
    pub catalogue_id: i32,
    pub tag_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Catalogue {
    pub id: i32,
    pub catalogue: String,
    pub info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CatalogueCreate {
    pub user_detail_id:i64,
    pub catalogue: String,
    pub info: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CatalogueUpdate {
    pub catalogue: String,
    pub info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct BlogCatalogueCreate {
    pub catalogue_id: i32,
    pub blog_id: i32,
    pub sort_order: i32,
}
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CatalogueArticleTitles {
    pub article_id: i32,
    pub title: String,
    pub digest: String,
    pub sort_order: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct AddCatalogueArticle {
    pub article_id: i32,
    pub catalogue_id: i32,
    pub sort_order: i32,
}
