use crate::dbs::article_db::*;
use crate::dbs::comment_db::*;
use crate::error::*;
use crate::models::comment::*;
use crate::models::parameter::*;
use crate::models::state::AppState;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse, Router};
use std::sync::Arc;
use tracing::{debug, error, info};

pub async fn post_comment(
    app_state: State<Arc<AppState>>,
    Json(comment_create): Json<CommentCreate>,
) -> Result<impl IntoResponse, AppError> {
    post_comment_db(&app_state.pool, &comment_create).await?;
    Ok(StatusCode::OK)
}

pub async fn get_comments_by_article_id(
    app_state: State<Arc<AppState>>,
    Path(article_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let comments_vec = get_comments_db_by_article_id(&app_state.pool, article_id).await?;
    Ok((StatusCode::OK, Json(comments_vec)))
}
