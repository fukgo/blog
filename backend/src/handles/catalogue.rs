use crate::dbs::article_db::*;
use crate::dbs::catalogue_db::*;
use crate::error::*;
use crate::models::catalogue::*;
use crate::models::parameter::*;
use crate::models::state::AppState;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse, Router};
use std::sync::Arc;
use tracing::{debug, error, info};

// 创建新目录
pub async fn post_catalogue(
    app_state: State<Arc<AppState>>,
    Json(catalogue_create): Json<CatalogueCreate>,
) -> Result<impl IntoResponse, AppError> {
    post_catalogue_db(&app_state.pool, catalogue_create).await?;
    Ok(StatusCode::OK)
}
// 更新目录
pub async fn post_update_catalogue(
    app_state: State<Arc<AppState>>,
    Path(catalogue_id): Path<i64>,
    Json(catalogue_update): Json<CatalogueUpdate>,
) -> Result<impl IntoResponse, AppError> {
    post_update_catalogue_db(&app_state.pool, catalogue_id, catalogue_update).await?;
    Ok(StatusCode::OK)
}
// 删除目录
pub async fn delete_catalogue(
    app_state: State<Arc<AppState>>,
    Path(catalogue_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    delete_catalogue_db(&app_state.pool, catalogue_id).await?;
    Ok(StatusCode::OK)
}

pub async fn get_all_catalogues(
    app_state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let catalogues = all_catalogues_db(&app_state.pool).await?;
    Ok((StatusCode::OK, Json(catalogues)))
}

pub async fn get_catalogue_by_id(
    app_state: State<Arc<AppState>>,
    Path(catalogue_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let catalogue = get_catalogue_by_id_db(&app_state.pool, catalogue_id).await?;
    Ok((StatusCode::OK, Json(catalogue)))
}
pub async fn get_catalogue_article_titles(
    app_state: State<Arc<AppState>>,
    Path(catalogue_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let articles = get_catalogue_article_titles_db(&app_state.pool, catalogue_id).await?;
    Ok((StatusCode::OK, Json(articles)))
}

//移除目录下的文章
pub async fn delete_catalogue_article_by_id(
    app_state: State<Arc<AppState>>,
    Path((catalogue_id, article_id)): Path<(i64, i64)>,
) -> Result<impl IntoResponse, AppError> {
    delete_catalogue_article_one_by_id(&app_state.pool, article_id, catalogue_id).await?;
    Ok(StatusCode::OK)
}

//移除目录下的所有文章
pub async fn delete_catalogue_all_articles(
    app_state: State<Arc<AppState>>,
    Path(catalogue_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    delete_catalogue_article_all_by_id(&app_state.pool, catalogue_id).await?;
    Ok(StatusCode::OK)
}

//添加文章到目录
pub async fn post_catalogue_article(
    app_state: State<Arc<AppState>>,
    Json(parameter): Json<AddCatalogueArticle>,

) -> Result<impl IntoResponse, AppError> {
    debug!("{:?}",parameter);
    post_article_to_catalogue(&app_state.pool, &parameter).await?;
    Ok(StatusCode::OK)
}

//更新目录下文章的排序
pub async fn post_catalogue_article_sort(
    app_state: State<Arc<AppState>>,
    Json(parameter): Json<AddCatalogueArticle>,
) -> Result<impl IntoResponse, AppError> {
    update_catalogue_article_sort_order_by_id(&app_state.pool, &parameter).await?;
    Ok(StatusCode::OK)
}
