use crate::models::catalogue::*;
use crate::models::comment::*;
use crate::models::parameter::*;
use core::error;
use std::collections::HashMap;

use crate::utils::get_now_date;
use crate::{error::AppError, models::user::*};

use chrono::{format, Utc};
use serde_json::de;
use sqlx::MySqlPool;
use sqlx::Row;
use tracing::{debug, error, info};

// 嵌套结构返回评论
pub async fn post_comment_db(
    pool: &MySqlPool,
    comment_create: &CommentCreate,
) -> Result<(), AppError> {
    // 通过查询一次获取 parent_depth
    let (depth, parent_id) = if let Some(parent_id) = comment_create.parent_id {
        let parent_depth =
            sqlx::query_scalar::<_, i32>(r#"SELECT depth FROM comments_table WHERE id = ?"#)
                .bind(parent_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| {
                    error!("get parent comment failed: {:?}", e);
                    AppError::InternalError
                })?;

        match parent_depth {
            Some(depth) => (depth + 1, Some(parent_id)),
            None => {
                error!("parent_id not found");
                return Err(AppError::InternalError);
            }
        }
    } else {
        (1, None) // 顶级评论的深度为 1
    };

    // 插入评论
    let res = sqlx::query(r#"INSERT INTO comments_table (guest, article_id, parent_id, comment, depth) VALUES (?, ?, ?, ?, ?)"#)
        .bind(&comment_create.guest)
        .bind(&comment_create.article_id)
        .bind(parent_id)
        .bind(&comment_create.comment)
        .bind(depth)
        .execute(pool)
        .await.map_err(|e| {
            error!("post comment failed: {:?}", e);
            AppError::InternalError
        })?;

    Ok(())
}

pub async fn get_comments_db_by_article_id(
    pool: &MySqlPool,
    article_id: i64,
) -> Result<Vec<CommentsDisplay>, AppError> {
    let comments =
        sqlx::query_as::<_, Comment>(r#"SELECT * FROM comments_table WHERE article_id = ?"#)
            .bind(article_id)
            .fetch_all(pool)
            .await
            .map_err(|e| {
                error!("get comments failed: {:?}", e);
                AppError::InternalError
            })?;

    debug!("comments: {:?}", comments);

    let mut comments_display: Vec<CommentsDisplay> = Vec::new();
    let mut comment_map: HashMap<i32, CommentsDisplay> = HashMap::new();

    // 将所有评论先放入 map 中
    for comment in &comments {
        let display_comment = CommentsDisplay::new_top_comment(comment.clone());
        comment_map.insert(comment.id, display_comment);
    }

    // 处理层级关系
    for comment in &comments {
        if let Some(parent_id) = comment.parent_id {
            if let Some(child_comment) = comment_map.remove(&comment.id) {
                // 找到父评论并添加子评论
                add_comment_to_parent(child_comment, &mut comments_display);
            } else {
                // 找不到父评论时记录错误
                error!("Parent comment with id {} not found.", parent_id);
            }
        } else {
            // 顶级评论直接添加到最终列表中
            comments_display.push(comment_map.remove(&comment.id).unwrap());
        }
    }

    Ok(comments_display)
}
