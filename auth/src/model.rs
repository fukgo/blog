use askama::Template;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::{MySql, Pool};
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,                   // BIGINT 对应 i64
    pub username: String,          // VARCHAR 对应 String
    pub password: String,          // VARCHAR 对应 String
    pub email: String,             // VARCHAR 对应 String
    pub created_at: DateTime<Utc>, // 使用 chrono 的 DateTime<Utc> 处理时间
    pub updated_at: DateTime<Utc>, // 使用 chrono 的 DateTime<Utc> 处理时间
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AuthedUser {
    pub id: i64,
    pub username: String,
    pub email: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    // user_detail: UserDetail,
}

#[derive(Debug, Serialize)]
pub struct MsgResponse {
    pub msg: String,
}

pub struct Params {
    pub key: String,
    pub timeout: u64,
}

pub struct AppState {
    pub pool: Pool<MySql>,
}

#[derive(Template, Deserialize, Serialize, Debug)]
#[template(path = "register.html")]
pub struct RegisterKey {
    pub authenticity_token: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_conform: String,
    pub access: String,
}

#[derive(Template, Deserialize, Serialize, Debug)]
#[template(path = "login.html")]
pub struct LoginKey {
    pub authenticity_token: String,
    pub username: String,
    pub password: String,
    pub redirect: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteList {
    pub id: i64,
    pub catalogue_id: i64,
    pub site_title: String,
    pub site_info: String,
    pub site_url: String,
    // pub site_icon: String,
    // pub site_category: String,
    // pub site_tags: String,
    // pub site_created_at: DateTime<Utc>,
    // pub site_updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SiteCatalogue {
    pub id: i64,
    pub catalogue_name: String,
    pub catalogue_info: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct SiteInfo {
    pub site_title: String,
    pub site_info: String,
    pub site_url: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct SiteVec {
    pub id: i32,
    pub category_name: String,
    pub web_list: Vec<SiteInfo>,
}

