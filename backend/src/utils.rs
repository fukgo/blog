use crate::error::AppError;
use crate::models::user::*;
use chrono::Datelike;
use reqwest::Client;
use reqwest::StatusCode;
use tracing::{debug, error};
pub async fn get_auth(token: &str, url: &str) -> Result<User, AppError> {
    let client = Client::new();
    let res = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    debug!("get user info response: {:?}", res);
    match res.status() {
        StatusCode::OK => {
            let user = res.json::<User>().await;
            debug!("get user info success: {:?}", user);
            match user {
                Ok(user) => {
                    debug!("get user info success:{} ", user.username);
                    Ok(user)
                }
                Err(e) => {
                    error!("get user info failed: {:?}", e);
                    Err(AppError::InternalError)
                }
            }
        }
        StatusCode::UNAUTHORIZED => {
            error!("token invalid");
            Err(AppError::TokenInvalid)
        }
        _ => {
            error!("get user info failed");
            Err(AppError::InternalError)
        }
    }
}
// 返回年份_月份，如2021_08
pub async fn get_now_date() -> String {
    // let now = chrono::Utc::now();
    // let year = now.year();
    // let month = now.month();
    // format!("{}_{}", year, month)
    "2024_10".to_string()
}

pub async fn get_table_name() -> String {
    let now_date = get_now_date().await;
    format!("articles_table_{}", now_date)
}
