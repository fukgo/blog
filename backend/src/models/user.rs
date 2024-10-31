use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};
use std::sync::Mutex;

// #[derive(Debug, Serialize, Deserialize,Clone,FromRow)]
// pub struct AuthedUser{
//     pub id: i64,
//     pub username: String,
//     pub email: String,
// }

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct UserDetail {
    pub id: i64,
    pub user_id: i64,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub skills: Option<String>,
    pub bio: Option<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct UserSession {
    pub user_detail_id: i64,
    pub username: String,
    pub email: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct UserInfo {
    pub id: i64,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct UserDetailUpdate {
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub skills: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Resume {
    pub id: i64,
    pub user_detail_id: i64,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResumeCreate {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResumeUpdate {
    pub content: String,
}

// id,
// email,
// username,
// avatar,
// nickname,
// skills
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct UserDetailDisplay {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub skills: Option<String>,
    pub bio: Option<String>,
}
