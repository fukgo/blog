use crate::models::article::ArticleTitle;
use crate::models::catalogue::*;
use crate::models::parameter::*;

use crate::utils::get_now_date;
use crate::{error::AppError, models::user::*};

use chrono::{format, Utc};
use sqlx::MySqlPool;
use sqlx::Row;
use tracing::{debug, error, info};

pub async fn all_catalogues_db(pool: &MySqlPool) -> Result<Vec<Catalogue>, AppError> {
    let res = sqlx::query_as::<_, Catalogue>(r#"SELECT * FROM catalogues_table"#)
        .fetch_all(pool)
        .await;

    match res {
        Ok(catalogues) => {
            debug!("get all catalogues success");
            Ok(catalogues)
        }
        Err(e) => {
            error!("get all catalogues failed: {:?}", e);
            Err(AppError::InternalError)
        }
    }
}

pub async fn post_update_catalogue_db(
    pool: &MySqlPool,
    catalogue_id: i64,
    catalogue_update: CatalogueUpdate,
) -> Result<(), AppError> {
    let res = sqlx::query(r#"UPDATE catalogues_table SET catalogue = ?, info = ? WHERE id = ?"#)
        .bind(catalogue_update.catalogue)
        .bind(catalogue_update.info)
        .bind(catalogue_id)
        .execute(pool)
        .await;

    match res {
        Ok(_) => {
            debug!("update catalogue success");
            Ok(())
        }
        Err(e) => {
            error!("update catalogue failed: {:?}", e);
            Err(AppError::InternalError)
        }
    }
}

pub async fn post_catalogue_db(
    pool: &MySqlPool,
    catalogue_creat: CatalogueCreate,
) -> Result<(), AppError> {
    let res = sqlx::query(r#"INSERT INTO catalogues_table (user_detail_id,catalogue,info) VALUES (?,?,?)"#)
        .bind(catalogue_creat.user_detail_id)
        .bind(catalogue_creat.catalogue)
        .bind(catalogue_creat.info)
        .execute(pool)
        .await;

    match res {
        Ok(_) => {
            debug!("post catalogue success");
            Ok(())
        }
        Err(e) => {
            error!("post catalogue failed: {:?}", e);
            Err(AppError::InternalError)
        }
    }
}

pub async fn delete_catalogue_db(pool: &MySqlPool, catalogue_id: i64) -> Result<(), AppError> {
    // 检查是否在 article_catalogues_table 中存在 catalogue_id
    let exists = sqlx::query_scalar::<_, bool>(
        r#"SELECT EXISTS(SELECT 1 FROM article_catalogues_table WHERE catalogue_id = ?)"#,
    )
    .bind(catalogue_id)
    .fetch_one(pool)
    .await?;

    if exists {
        error!("catalogue_id is in article_catalogues_table, can't delete");
        return Err(AppError::InternalError);
    }

    debug!("catalogue_id is not in article_catalogues_table, can delete");

    // 删除标签
    let delete_res = sqlx::query(r#"DELETE FROM catalogues_table WHERE id = ?"#)
        .bind(catalogue_id)
        .execute(pool)
        .await
        .map_err(|e| {
            error!("delete catalogue failed: {:?}", e);
            AppError::InternalError
        })?;

    match delete_res.rows_affected() {
        0 => {
            error!("delete catalogue failed: no row affected");
            Err(AppError::InternalError)
        }
        _ => {
            debug!("delete catalogue success");
            Ok(())
        }
    }
}

pub async fn get_catalogue_by_id_db(
    pool: &MySqlPool,
    catalogue_id: i64,
) -> Result<Catalogue, AppError> {
    let catalogue =
        sqlx::query_as::<_, Catalogue>(r#"SELECT * FROM catalogues_table WHERE id = ?"#)
            .bind(catalogue_id)
            .fetch_one(pool)
            .await;

    match catalogue {
        Ok(catalogue) => {
            debug!("get catalogue info success");
            Ok(catalogue)
        }
        Err(e) => {
            error!("get catalogue info failed: {:?}", e);
            Err(AppError::CatalogueNotFound)
        }
    }
}

pub async fn get_catalogue_article_titles_db(
    pool: &MySqlPool,
    catalogue_id: i64,
) -> Result<Vec<CatalogueArticleTitles>, AppError> {
    let vec = sqlx::query_as::<_, CatalogueArticleTitles>(
        r#"
SELECT a.id as article_id,a.title,a.digest,ac.sort_order
FROM articles_table_2024_10 AS a
JOIN article_catalogues_table AS ac ON a.id = ac.article_id
WHERE ac.catalogue_id = ?
ORDER BY ac.sort_order;
        "#,
    )
    .bind(catalogue_id)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!("get catalogue article titles failed: {:?}", e);
        AppError::InternalError
    })?;
    Ok(vec)
}

//移除目录下的文章
pub async fn delete_catalogue_article_one_by_id(
    pool: &MySqlPool,
    article_id: i64,
    catalogue_id: i64,
) -> Result<(), AppError> {
    let res = sqlx::query(
        r#"DELETE FROM article_catalogues_table WHERE article_id = ? AND catalogue_id = ?"#,
    )
    .bind(article_id)
    .bind(catalogue_id)
    .execute(pool)
    .await;

    match res {
        Ok(_) => {
            debug!("delete catalogue article success");
            Ok(())
        }
        Err(e) => {
            error!("delete catalogue article failed: {:?}", e);
            Err(AppError::InternalError)
        }
    }
}

//移除目录下的所有文章
pub async fn delete_catalogue_article_all_by_id(
    pool: &MySqlPool,
    catalogue_id: i64,
) -> Result<(), AppError> {
    let res = sqlx::query(r#"DELETE FROM article_catalogues_table WHERE catalogue_id = ?"#)
        .bind(catalogue_id)
        .execute(pool)
        .await;

    match res {
        Ok(_) => {
            debug!("delete catalogue article success");
            Ok(())
        }
        Err(e) => {
            error!("delete catalogue article failed: {:?}", e);
            Err(AppError::InternalError)
        }
    }
}

//添加文章到目录
pub async fn post_article_to_catalogue(
    pool: &MySqlPool,
    parameter: &AddCatalogueArticle,
) -> Result<(), AppError> {
    let res = sqlx::query(r#"INSERT INTO article_catalogues_table (article_id,catalogue_id,sort_order) VALUES (?,?,?)"#)
        .bind(parameter.article_id)
        .bind(parameter.catalogue_id)
        .bind(parameter.sort_order)
        .execute(pool)
        .await;

    match res {
        Ok(_) => {
            debug!("post article to catalogue success");
            Ok(())
        }
        Err(e) => {
            error!("post article to catalogue failed: {:?}", e);
            Err(AppError::InternalError)
        }
    }
}

//更新目录下文章的排序
pub async fn update_catalogue_article_sort_order_by_id(
    pool: &MySqlPool,
    parameter: &AddCatalogueArticle,
) -> Result<(), AppError> {
    let res = sqlx::query(r#"UPDATE article_catalogues_table SET sort_order = ? WHERE article_id = ? AND catalogue_id = ?"#)
        .bind(parameter.sort_order) 
        .bind(parameter.article_id)
        .bind(parameter.catalogue_id)
        .execute(pool)
        .await;

    match res {
        Ok(_) => {
            debug!("update catalogue article sort order success");
            Ok(())
        }
        Err(e) => {
            error!("update catalogue article sort order failed: {:?}", e);
            Err(AppError::InternalError)
        }
    }
}
