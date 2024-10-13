use axum::{
    response::IntoResponse,
    http::StatusCode,
    Router,
};
use axum::Json;
use tower_sessions::Session;
use tracing::{info,debug,error};
use crate::model::*;
use crate::error::*;
use core::error;
use std::sync::Arc;
use axum::extract::State;
use axum::extract::Request;
use axum::body::Body;
use axum::extract::Path;
use crate::utils::get_auth;
use crate::dbs::tag_db::*;
use axum::extract::Query;

// 创建新标签
pub async fn creata_tag(
    app_state: State<Arc<AppState>>,
    Path(tag_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    post_tag_db(&app_state.pool, &tag_name).await?;
    Ok(StatusCode::OK)
}
// 删除指定标签
pub async fn delete_tag(
    app_state: State<Arc<AppState>>,
    Path(tag_id): Path<i64>

) -> Result<impl IntoResponse, AppError> {
    delete_tag_db(&app_state.pool,tag_id).await?;
    Ok(StatusCode::OK)
}
// 获取所有标签
pub async fn get_tags(
    app_state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let tags = all_tags_db(&app_state.pool).await?;
    Ok((StatusCode::OK, Json(tags)))
}