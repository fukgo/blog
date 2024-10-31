use crate::dbs::article_db::get_article_tags_by_id;
use crate::dbs::tag_db::*;
use crate::error::*;
use crate::models::parameter::*;
use crate::models::state::*;
use crate::models::tag::*;
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse, Router};
use std::sync::Arc;
use tracing::{debug, error, info};

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
    Path(tag_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    delete_tag_db(&app_state.pool, tag_id).await?;
    Ok(StatusCode::OK)
}
// 获取所有标签
pub async fn get_tags(app_state: State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    let tags = all_tags_db(&app_state.pool).await?;
    Ok((StatusCode::OK, Json(tags)))
}

pub async fn get_article_tags(
    app_state: State<Arc<AppState>>,
    Path(article_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let tags = get_article_tags_by_id(&app_state.pool, article_id).await?;
    Ok((StatusCode::OK, Json(tags)))
}
