use crate::model::*;
use crate::utils::get_now_date;
use crate::{error::AppError, model::AuthedUser};
use sqlx::MySqlPool;
use tracing::{debug, error, info};

pub async fn all_tags_db(pool: &MySqlPool) -> Result<Vec<Tag>, AppError> {
    let res = sqlx::query_as::<_, Tag>(r#"SELECT * FROM tags_table"#)
        .fetch_all(pool)
        .await;

    match res {
        Ok(tags) => {
            debug!("get all tags success");
            Ok(tags)
        }
        Err(e) => {
            error!("get all tags failed: {:?}", e);
            Err(AppError::InternalError)
        }
    }
}
pub async fn post_tag_db(pool: &MySqlPool, tag: &str) -> Result<(), AppError> {
    let res = sqlx::query(r#"INSERT INTO tags_table (tag) VALUES (?)"#)
        .bind(tag)
        .execute(pool)
        .await;

    match res {
        Ok(_) => {
            debug!("post tag success");
            Ok(())
        }
        Err(e) => {
            error!("post tag failed: {:?}", e);
            Err(AppError::InternalError)
        }
    }
}
pub async fn delete_tag_db(pool: &MySqlPool, tag_id: i64) -> Result<(), AppError> {
    // 检查是否在 blog_tags_table 中存在 tag_id
    let exists = sqlx::query_scalar::<_, bool>(r#"SELECT EXISTS(SELECT 1 FROM blog_tags_table WHERE tag_id = ?)"#)
        .bind(tag_id)
        .fetch_one(pool)
        .await?;

    if exists {
        error!("tag_id is in blog_tags_table, can't delete");
        return Err(AppError::InternalError);
    }

    debug!("tag_id is not in blog_tags_table, can delete");

    // 删除标签
    sqlx::query(r#"DELETE FROM tags_table WHERE id = ?"#)
        .bind(tag_id)
        .execute(pool)
        .await
        .map_err(|e| {
            error!("delete tag failed: {:?}", e);
            AppError::InternalError
        })?;

    debug!("delete tag success");
    Ok(())
}
