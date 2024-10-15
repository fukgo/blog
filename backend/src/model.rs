use sqlx::{prelude::FromRow, MySqlPool};
use std::sync::Mutex;
use serde::{Serialize,Deserialize};
use chrono::Utc;
#[derive(Debug)]
pub struct AppState {
    pub pool:  MySqlPool,
    pub user_vec: Mutex<Vec<AuthedUser>>,
}
#[derive(Debug, Serialize, Deserialize,Clone,FromRow)]
pub struct AuthedUser{
    pub id: i64,                
    pub username: String,       
    pub email: String, 
}
impl PartialEq for AuthedUser {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
    }
}
#[derive(Debug, Serialize, Deserialize,Clone,FromRow)]
pub struct User{
    pub id: i64,
    pub username: String,
    pub email: String,
    pub nickname: Option<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize,Clone,FromRow)]
pub struct Tag{
    pub id: i64,
    pub tag: String,
}
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct TagCreate{
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,FromRow)]
pub struct Article{
    pub id: i64,
    pub title: String,
    pub content: String,
    pub digest: String,
    pub user_id: i64,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ArticleDisplay{
    pub id: i64,
    pub title: String,
    pub digest: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
    pub tags: Vec<String>,
    pub author: User,
    pub total_page: i64,

}

// #[derive(Debug, Serialize, Deserialize,Clone)]
// pub struct ArticleDetail{
//     pub id: i64,
//     pub title: String,
//     pub content: String,
//     pub digest: String,
//     pub created_at: chrono::DateTime<Utc>,
//     pub updated_at: chrono::DateTime<Utc>,
//     pub tags: Vec<String>,
//     pub author: User,
// }
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ArticleCreate{
    pub title: String,
    pub content: String,
    pub digest: String,
    pub user_id: i64,
    pub tags_id: Vec<i64>,
}
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ArticleUpdate{
    pub title: Option<String>,
    pub content: Option<String>,
    pub digest: Option<String>,
}

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
#[derive(Debug, Serialize, Deserialize,Clone,FromRow)]
pub struct BlogTag{
    pub id: i64,
    pub blog_id: i64,
    pub tag_id: i64,
}

#[derive(Debug, Serialize, Deserialize,Clone,FromRow)]
pub struct Resume{
    pub id: i64,
    pub user_id: i64,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct ResumeCreate{
    pub user_id: i64,
    pub content: String,
}

