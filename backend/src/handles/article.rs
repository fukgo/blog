use crate::dbs::article_db::*;
use crate::error::*;
use crate::models::article::*;
use crate::models::parameter::*;
use crate::models::state::*;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse, Router};
use std::sync::Arc;
use tracing::{debug, error, info};

// 创建新文章
pub async fn post_article(
    app_state: State<Arc<AppState>>,
    Json(article_create): Json<ArticleCreate>,
) -> Result<impl IntoResponse, AppError> {
    debug!("article_create: {:?}", article_create);
    post_article_db(&app_state.pool, &article_create).await?;
    Ok(StatusCode::OK)
}
// 更新指定文章
pub async fn put_article(
    app_state: State<Arc<AppState>>,
    Path(article_id): Path<i64>,
    Json(article): Json<ArticleUpdate>,
) -> Result<impl IntoResponse, AppError> {
    update_article_db(&app_state.pool, article_id, &article).await?;
    Ok(StatusCode::OK)
}

pub async fn get_article_detail(
    app_state: State<Arc<AppState>>,
    Path(article_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let articles_vec = get_article_detail_db(&app_state.pool, article_id).await?;
    Ok((StatusCode::OK, Json(articles_vec)))
}
// 获取用户的指定文章
pub async fn get_user_article(
    app_state: State<Arc<AppState>>,
    Path(user_id): Path<i64>,
    Query(param): Query<PageParams>,
) -> Result<impl IntoResponse, AppError> {
    let articles_vec = get_article_info_by_userid(&app_state.pool, user_id, &param).await?;
    Ok((StatusCode::OK, Json(articles_vec)))
}
// 分页获取特定标签下的文章信息
pub async fn get_tag_articles_info(
    app_state: State<Arc<AppState>>,
    Path(tag_id): Path<i64>,
    Query(param): Query<PageParams>,
) -> Result<impl IntoResponse, AppError> {
    let articles_vec = get_article_info_by_tagid(&app_state.pool, tag_id, &param).await?;
    Ok((StatusCode::OK, Json(articles_vec)))
}
// 删除指定文章
pub async fn delete_article(
    app_state: State<Arc<AppState>>,
    Path(article_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    delete_article_db(&app_state.pool, article_id).await?;
    Ok(StatusCode::OK)
}

pub async fn update_article(
    app_state: State<Arc<AppState>>,
    Path(article_id): Path<i64>,
    Json(article_update): Json<ArticleUpdate>,
) -> Result<impl IntoResponse, AppError> {
    update_article_db(&app_state.pool, article_id, &article_update).await?;
    Ok(StatusCode::OK)
}

pub async fn get_featured_article(
    app_state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let articles_vec = get_featured_article_info(&app_state.pool).await?;
    Ok((StatusCode::OK, Json(articles_vec)))
}

pub async fn get_articles_latest(
    app_state: State<Arc<AppState>>,
    Query(param): Query<PageParams>,
) -> Result<impl IntoResponse, AppError> {
    let articles_vec = get_articles_last_db(&app_state.pool, &param).await?;
    Ok((StatusCode::OK, Json(articles_vec)))
}

pub async fn get_article_titles(
    app_state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let articles_vec = get_articles_titles(&app_state.pool).await?;
    Ok((StatusCode::OK, Json(articles_vec)))
}
