use crate::error::AppError;
use crate::model::AuthedUser;
use reqwest::StatusCode;
use reqwest::Client;
use chrono::Datelike;
use tracing::{error,debug};
pub async fn get_auth(
    token: &str,
    url: &str,
) -> Result<AuthedUser, AppError> {
    let client = Client::new();
    let res = client.get(url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await.unwrap();
    debug!("get user info response: {:?}", res);
    match res.status() {
        StatusCode::OK => {
            let user = res.json::<AuthedUser>().await;
            debug!("get user info success: {:?}", user);
            match user{
                Ok(user) => {
                    debug!("get user info success:{} ", user.username);
                    Ok(user)
                },
                Err(e) => {
                    error!("get user info failed: {:?}", e);
                    Err(AppError::InternalError)
                }
            }
        },
        StatusCode::UNAUTHORIZED => {
            error!("token invalid");
            Err(AppError::TokenInvalid)
        },
        _ => {
            error!("get user info failed");
            Err(AppError::InternalError)
        },
    }
}
// 返回年份_月份，如2021_08
pub async fn get_now_date() -> String {
    let now = chrono::Utc::now();
    let year = now.year();
    let month = now.month();
    format!("{}_{}", year, month)
}
