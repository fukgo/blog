use crate::handles::{article, user};
use crate::models::article::*;
use crate::models::parameter::*;
use crate::models::user::*;
use crate::utils::get_table_name;
use crate::{error::AppError, models::user::*};
use chrono::{format, Utc};
use sqlx::MySqlPool;
use sqlx::Row;
use tracing::{debug, error, info};
pub async fn post_article_db(pool: &MySqlPool, article: &ArticleCreate) -> Result<(), AppError> {
    let mut conn = pool.begin().await.map_err({
        |e| {
            error!("begin transaction failed: {:?}", e);
            AppError::InternalError
        }
    })?;
    let article_table_name = get_table_name().await;
    let query = format!(
        r#"INSERT INTO {} (title, content,digest, user_detail_id,feature) VALUES (?, ?, ?,?,?)"#,
        article_table_name
    );
    let article_res = sqlx::query(&query)
        .bind(&article.title)
        .bind(&article.content)
        .bind(&article.digest)
        .bind(&article.user_detail_id)
        .bind(&article.feature)
        .execute(&mut *conn)
        .await;
    match article_res {
        Ok(result) => {
            let last_insert_id = result.last_insert_id();
            // debug!("post article success, id: {}", last_insert_id);

            for tag_id in &article.tags_id {
                let res = sqlx::query(
                    r#"INSERT INTO article_tags_table (article_id, tag_id) VALUES (?, ?)"#,
                )
                .bind(last_insert_id)
                .bind(tag_id)
                .execute(&mut *conn)
                .await;

                if let Err(e) = res {
                    error!("post article tag failed: {:?}", e);
                    // 回滚事务
                    conn.rollback().await.map_err(|e| {
                        error!("rollback transaction failed: {:?}", e);
                        AppError::InternalError
                    })?;
                    return Err(AppError::InternalError);
                } else {
                    debug!("post article tag success");
                }
            }
            // 提交事务
            conn.commit().await.map_err(|e| {
                error!("commit transaction failed: {:?}", e);
                AppError::InternalError
            })?;
            Ok(())
        }
        Err(e) => {
            error!("post article failed: {:?}", e);
            // 回滚事务
            conn.rollback().await.map_err(|e| {
                error!("rollback transaction failed: {:?}", e);
                AppError::InternalError
            })?;
            Err(AppError::InternalError)
        }
    }
}
pub async fn get_article_detail_db(pool: &MySqlPool, article_id: i64) -> Result<Article, AppError> {
    // 返回类型修改为包含单个文章和总页数
    let article_table_name = get_table_name().await;
    let query = format!(r#"select * from {} where id = ?"#, article_table_name);
    let article = sqlx::query_as::<_, Article>(&query)
        .bind(article_id)
        .fetch_one(pool)
        .await?;

    Ok(article)
}

pub async fn get_article_info_by_userid(
    pool: &MySqlPool,
    tag_id: i64,
    param: &PageParams,
) -> Result<Vec<ArticleDisplay>, AppError> {
    let article_table_name = get_table_name().await;
    let page = param.page.unwrap_or(1);
    let limit = param.limit.unwrap_or(10);
    let offset = (page - 1) * limit;
    // 查询满足条件的文章总数
    let total_count_query = format!(
        r#"
          SELECT COUNT(*) AS total
          FROM
              {} b
          WHERE
              b.user_detail_id = ?;  -- 添加对 user_detail_id 的过滤
          "#,
        article_table_name
    );
    let total_count_row = sqlx::query(&total_count_query)
        .bind(tag_id) // 绑定 tag_id
        .fetch_one(pool)
        .await
        .map_err(|e| {
            error!("get total count failed: {:?}", e);
            AppError::InternalError
        })?;
    let total_count: i64 = total_count_row.get("total");
    let total_page = (total_count as f64 / limit as f64).ceil() as i64; // 计算总页数
                                                                        // 查询文章信息
    let query = format!(
        r#"
        SELECT
            b.id AS article_id,
            b.title AS article_title,
            b.digest AS article_digest,
            b.feature AS article_feature,
            b.created_at AS article_created_at,
            b.updated_at AS article_updated_at,
            u.id AS author_id,
            u.nickname AS author_nickname,
            u.avatar AS author_avatar,
            GROUP_CONCAT(t.tag ORDER BY t.tag SEPARATOR ', ') AS tags
        FROM
            {} b
        LEFT JOIN
            user_detail_table u ON b.user_detail_id = u.id
        LEFT JOIN
            article_tags_table bt ON b.id = bt.article_id
        LEFT JOIN
            tags_table t ON bt.tag_id = t.id
        WHERE
            u.id = ?  -- 过滤出与指定 tag_id 相关的文章
        GROUP BY
            b.id, u.id
        LIMIT ? OFFSET ?;  -- 使用 LIMIT 和 OFFSET 实现分页
        "#,
        article_table_name
    );
    let rows = sqlx::query(&query)
        .bind(tag_id) // 绑定 tag_id
        .bind(limit) // 绑定 limit
        .bind(offset) // 绑定 offset
        .fetch_all(pool)
        .await
        .map_err(|e| {
            error!("get article info failed: {:?}", e);
            AppError::InternalError
        })?;
    // 将结果转换为 ArticleDetail 结构
    let articles_info: Vec<ArticleDisplay> = rows
        .into_iter()
        .map(|row| {
            let tags_str: Option<String> = row.get("tags");
            let tags: Vec<String> = tags_str
                .unwrap_or_default()
                .split(", ")
                .map(String::from)
                .collect();
            ArticleDisplay {
                id: row.get::<i64, _>("article_id"),
                title: row.get::<String, _>("article_title"),
                // content: row.get::<String, _>("article_content"),
                digest: row.get::<String, _>("article_digest"),
                feature: row.get::<bool, _>("article_feature"),
                created_at: row.get::<chrono::DateTime<Utc>, _>("article_created_at"),
                updated_at: row.get::<chrono::DateTime<Utc>, _>("article_updated_at"),
                tags, // 包含标签名称
                author: UserInfo {
                    id: row.get::<i64, _>("author_id"),
                    nickname: Some(row.get::<String, _>("author_nickname")),
                    avatar: Some(row.get::<String, _>("author_avatar")),
                },
                total_page: Some(total_page), // 包含总页数
            }
        })
        .collect();
    Ok(articles_info) // 返回文章和总页数
}

pub async fn get_article_info_by_tagid(
    pool: &MySqlPool,
    tag_id: i64,
    param: &PageParams,
) -> Result<Vec<ArticleDisplay>, AppError> {
    // 修改返回类型以包含总页数
    let article_table_name = get_table_name().await;
    let page = param.page.unwrap_or(1);
    let limit = param.limit.unwrap_or(10);
    let offset = (page - 1) * limit;
    // 查询满足条件的文章总数
    let total_count_query = format!(
        r#"
        SELECT COUNT(*) AS total
        FROM
            {} b
        LEFT JOIN
            article_tags_table bt ON b.id = bt.article_id
        WHERE
            bt.tag_id = ?;  -- 过滤出与指定 tag_id 相关的文章
        "#,
        article_table_name
    );
    let total_count_row = sqlx::query(&total_count_query)
        .bind(tag_id) // 绑定 tag_id
        .fetch_one(pool)
        .await
        .map_err(|e| {
            error!("get total count failed: {:?}", e);
            AppError::InternalError
        })?;
    let total_count: i64 = total_count_row.get("total");
    let total_page = (total_count as f64 / limit as f64).ceil() as i64; // 计算总页数
                                                                        // 查询文章信息
    let query = format!(
        r#"
        SELECT
            b.id AS article_id,
            b.title AS article_title,
            b.digest AS article_digest,
            b.feature AS article_feature,
            b.created_at AS article_created_at,
            b.updated_at AS article_updated_at,
            u.id AS author_id,
            u.nickname AS author_nickname,
            u.avatar AS author_avatar,
            GROUP_CONCAT(t.tag ORDER BY t.tag SEPARATOR ', ') AS tags
        FROM
            {} b
        LEFT JOIN
            user_detail_table u ON b.user_detail_id = u.id
        LEFT JOIN
            article_tags_table bt ON b.id = bt.article_id
        LEFT JOIN
            tags_table t ON bt.tag_id = t.id
        WHERE
            bt.tag_id = ?  -- 过滤出与指定 tag_id 相关的文章
        GROUP BY
            b.id, u.id
        LIMIT ? OFFSET ?;  -- 使用 LIMIT 和 OFFSET 实现分页
        "#,
        article_table_name
    );
    let rows = sqlx::query(&query)
        .bind(tag_id) // 绑定 tag_id
        .bind(limit) // 绑定 limit
        .bind(offset) // 绑定 offset
        .fetch_all(pool)
        .await
        .map_err(|e| {
            error!("get article info failed: {:?}", e);
            AppError::InternalError
        })?;
    // 将结果转换为 ArticleDetail 结构
    let articles_info: Vec<ArticleDisplay> = rows
        .into_iter()
        .map(|row| {
            let tags_str: Option<String> = row.get("tags");
            let tags: Vec<String> = tags_str
                .unwrap_or_default()
                .split(", ")
                .map(String::from)
                .collect();
            ArticleDisplay {
                id: row.get::<i64, _>("article_id"),
                title: row.get::<String, _>("article_title"),
                // content: row.get::<String, _>("article_content"),
                digest: row.get::<String, _>("article_digest"),
                feature: row.get::<bool, _>("article_feature"),
                created_at: row.get::<chrono::DateTime<Utc>, _>("article_created_at"),
                updated_at: row.get::<chrono::DateTime<Utc>, _>("article_updated_at"),
                tags, // 包含标签名称
                author: UserInfo {
                    id: row.get::<i64, _>("author_id"),
                    nickname: Some(row.get::<String, _>("author_nickname")),
                    avatar: Some(row.get::<String, _>("author_avatar")),
                },
                total_page: Some(total_page), // 包含总页数
            }
        })
        .collect();
    Ok(articles_info) // 返回文章和总页数
}

pub async fn delete_article_db(pool: &MySqlPool, article_id: i64) -> Result<(), AppError> {
    // Start a transaction
    let mut conn = pool.begin().await.map_err(|e| {
        error!("begin transaction failed: {:?}", e);
        AppError::InternalError
    })?;

    let article_table_name = get_table_name().await;

    // Delete the article from the specified table
    let res = sqlx::query(&format!(
        r#"DELETE FROM {} WHERE id = ?"#,
        article_table_name
    ))
    .bind(article_id)
    .execute(&mut *conn)
    .await;

    // Check if the delete operation was successful
    match res {
        Ok(_) => {
            debug!("delete article success");
            // Attempt to delete related entries in the article_tags_table
            let res = sqlx::query(r#"DELETE FROM article_tags_table WHERE article_id = ?"#)
                .bind(article_id)
                .execute(&mut *conn)
                .await;

            // Check if the delete operation for article_tags_table was successful
            if res.is_ok() {
                debug!("delete article_tags_table success");
                // Commit the transaction if both deletions were successful
                conn.commit().await.map_err(|e| {
                    error!("commit transaction failed: {:?}", e);
                    AppError::InternalError
                })?;
                return Ok(());
            } else {
                // Log the error and roll back the transaction if deletion fails
                error!("delete article_tags_table failed: {:?}", res);
                conn.rollback().await.map_err(|e| {
                    error!("rollback transaction failed: {:?}", e);
                    AppError::InternalError
                })?;
                return Err(AppError::InternalError);
            }
        }
        Err(e) => {
            // Log the error if the deletion of the article fails
            error!("delete article failed: {:?}", e);
            // Roll back the transaction
            conn.rollback().await.map_err(|e| {
                error!("rollback transaction failed: {:?}", e);
                AppError::InternalError
            })?;
            Err(AppError::InternalError)
        }
    }
}

pub async fn update_article_db(
    pool: &MySqlPool,
    article_id: i64,
    article: &ArticleUpdate,
) -> Result<(), AppError> {
    let article_table_name = get_table_name().await;

    // 创建一个可变的查询构建器
    let mut query = format!("UPDATE {} SET ", article_table_name);
    let mut updates = Vec::new(); // 用于存储字段更新的片段

    // 检查每个字段是否为空，如果不为空则添加到更新语句中
    if let Some(ref title) = article.title {
        updates.push("title = ?".to_string());
    }
    if let Some(ref content) = article.content {
        updates.push("content = ?".to_string());
    }
    if let Some(ref digest) = article.digest {
        updates.push("digest = ?".to_string());
    }
    if let Some(ref feature) = article.feature {
        updates.push("feature = ?".to_string());
    }

    // 检查是否有任何字段需要更新
    if updates.is_empty() {
        debug!("No fields to update for article_id: {}", article_id);
        return Ok(()); // 没有字段需要更新，直接返回成功
    }

    // 将更新的字段组合成完整的查询语句
    query += &updates.join(", "); // 以逗号连接各个更新片段
    query += " WHERE id = ?"; // 添加条件

    // 创建查询并绑定值
    let mut sql_query = sqlx::query(&query); // 使用不同的变量名
    if let Some(ref title) = article.title {
        sql_query = sql_query.bind(title);
    }
    if let Some(ref content) = article.content {
        sql_query = sql_query.bind(content);
    }
    if let Some(ref digest) = article.digest {
        sql_query = sql_query.bind(digest);
    }
    if let Some(ref feature) = article.feature {
        sql_query = sql_query.bind(feature);
    }
    sql_query = sql_query.bind(article_id); // 绑定 article_id
    let mut conn = pool.begin().await.map_err({
        |e| {
            error!("begin transaction failed: {:?}", e);
            AppError::InternalError
        }
    })?;
    // 执行查询
    let res = sql_query.execute(pool).await;

    match res {
        Ok(r) => {
            debug!("update article success");
            if let Some(tags_vec) = &article.tags_id {
                for tag_id in tags_vec {
                    let res = sqlx::query(
                        r#"INSERT INTO article_tags_table (article_id, tag_id) VALUES (?, ?)"#,
                    )
                    .bind(article_id)
                    .bind(tag_id)
                    .execute(&mut *conn)
                    .await;

                    if let Err(e) = res {
                        error!("post article tag failed: {:?}", e);
                        // 回滚事务
                        conn.rollback().await.map_err(|e| {
                            error!("rollback transaction failed: {:?}", e);
                            AppError::InternalError
                        })?;
                        return Err(AppError::InternalError);
                    } else {
                        debug!("post article tag success");
                    }
                }
                // 提交事务
                conn.commit().await.map_err(|e| {
                    error!("commit transaction failed: {:?}", e);
                    AppError::InternalError
                })?;
            }

            Ok(())
        }
        Err(e) => {
            error!("post article failed: {:?}", e);
            // 回滚事务
            conn.rollback().await.map_err(|e| {
                error!("rollback transaction failed: {:?}", e);
                AppError::InternalError
            })?;
            Err(AppError::InternalError)
        }
    }
}
pub async fn get_featured_article_info(pool: &MySqlPool) -> Result<Vec<ArticleDisplay>, AppError> {
    let article_table_name = get_table_name().await;

    // 查询文章信息，随机排序并限制返回数量为 6
    let query = format!(
        r#"
        SELECT
            b.id AS article_id,
            b.title AS article_title,
            b.digest AS article_digest,
            b.feature AS article_feature,
            b.created_at AS article_created_at,
            b.updated_at AS article_updated_at,
            u.id AS author_id,
            u.nickname AS author_nickname,
            u.avatar AS author_avatar,
            GROUP_CONCAT(t.tag ORDER BY t.tag SEPARATOR ', ') AS tags
        FROM
            {} b
        LEFT JOIN
            user_detail_table u ON b.user_detail_id = u.id
        LEFT JOIN
            article_tags_table bt ON b.id = bt.article_id
        LEFT JOIN
            tags_table t ON bt.tag_id = t.id
        WHERE
            b.feature = true  -- 过滤出 feature 为 true 的文章
        GROUP BY
            b.id, u.id
        ORDER BY RAND()  -- 随机排序
        LIMIT 6;  -- 限制返回数量为 6
        "#,
        article_table_name
    );
    let rows = sqlx::query(&query).fetch_all(pool).await.map_err(|e| {
        error!("get article info failed: {:?}", e);
        AppError::InternalError
    })?;

    // 将结果转换为 ArticleDisplay 结构
    let articles_info: Vec<ArticleDisplay> = rows
        .into_iter()
        .map(|row| {
            let tags_str: Option<String> = row.get("tags");
            let tags: Vec<String> = tags_str
                .unwrap_or_default()
                .split(", ")
                .map(String::from)
                .collect();
            ArticleDisplay {
                id: row.get::<i64, _>("article_id"),
                title: row.get::<String, _>("article_title"),
                digest: row.get::<String, _>("article_digest"),
                feature: row.get::<bool, _>("article_feature"),
                created_at: row.get::<chrono::DateTime<Utc>, _>("article_created_at"),
                updated_at: row.get::<chrono::DateTime<Utc>, _>("article_updated_at"),
                tags, // 包含标签名称
                author: UserInfo {
                    id: row.get::<i64, _>("author_id"),
                    nickname: Some(row.get::<String, _>("author_nickname")),
                    avatar: Some(row.get::<String, _>("author_avatar")),
                },
                total_page: None, // 设置 total_page 为 None
            }
        })
        .collect();

    Ok(articles_info) // 返回文章信息
}

pub async fn get_articles_last_db(
    pool: &MySqlPool,
    param: &PageParams,
) -> Result<Vec<ArticleDisplay>, AppError> {
    let article_table_name = get_table_name().await;
    let page = param.page.unwrap_or(1);
    let limit = param.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    // 查询满足条件的文章总数
    let total_count_query = format!(
        r#"
        SELECT COUNT(*) AS total
        FROM
            {} b;  
        "#,
        article_table_name
    );
    let total_count_row = sqlx::query(&total_count_query).fetch_one(pool).await?;
    let total_count: i64 = total_count_row.get("total");
    let total_page = (total_count as f64 / limit as f64).ceil() as i64; // 计算总页数

    // 查询文章信息
    let query = format!(
        r#"
        SELECT
            b.id AS article_id,
            b.title AS article_title,
            b.digest AS article_digest,
            b.feature AS article_feature,
            b.created_at AS article_created_at,
            b.updated_at AS article_updated_at,
            u.id AS author_id,
            u.nickname AS author_nickname,
            u.avatar AS author_avatar,
            GROUP_CONCAT(t.tag ORDER BY t.tag SEPARATOR ', ') AS tags
        FROM
              {} b
        LEFT JOIN
            user_detail_table u ON b.user_detail_id = u.id
        LEFT JOIN
            article_tags_table bt ON b.id = bt.article_id
        LEFT JOIN
            tags_table t ON bt.tag_id = t.id
        GROUP BY
            b.id, u.id,b.created_at
        ORDER BY b.created_at DESC 
        LIMIT ? OFFSET ?;  -- 使用 LIMIT 和 OFFSET 实现分页
        "#,
        article_table_name
    );
    let rows = sqlx::query(&query)
        .bind(limit) // 绑定 limit
        .bind(offset) // 绑定 offset
        .fetch_all(pool)
        .await
        .map_err(|e| {
            error!("get article info failed: {:?}", e);
            AppError::InternalError
        })?;

    // 将结果转换为 ArticleDisplay 结构
    let articles_info: Vec<ArticleDisplay> = rows
        .into_iter()
        .map(|row| {
            let tags_str: Option<String> = row.get("tags");
            let tags: Vec<String> = tags_str
                .unwrap_or_default()
                .split(", ")
                .map(String::from)
                .collect();
            ArticleDisplay {
                id: row.get::<i64, _>("article_id"),
                title: row.get::<String, _>("article_title"),
                digest: row.get::<String, _>("article_digest"),
                feature: row.get::<bool, _>("article_feature"),
                created_at: row.get::<chrono::DateTime<Utc>, _>("article_created_at"),
                updated_at: row.get::<chrono::DateTime<Utc>, _>("article_updated_at"),
                tags, // 包含标签名称
                author: UserInfo {
                    id: row.get::<i64, _>("author_id"),
                    nickname: Some(row.get::<String, _>("author_nickname")),
                    avatar: Some(row.get::<String, _>("author_avatar")),
                },
                total_page: Some(total_page), // 包含总页数
            }
        })
        .collect();

    Ok(articles_info) // 返回文章和总页数
}

pub async fn get_articles_titles(pool: &MySqlPool) -> Result<Vec<ArticleTitle>, AppError> {
    let res = sqlx::query_as::<_, ArticleTitle>(r#"SELECT id,title FROM articles_table_2024_10"#)
        .fetch_all(pool)
        .await;

    match res {
        Ok(articles) => {
            debug!("get all articles success");
            Ok(articles)
        }
        Err(e) => {
            error!("get all articles failed: {:?}", e);
            Err(AppError::InternalError)
        }
    }
}

pub async fn get_article_tags_by_id(
    pool: &MySqlPool,
    article_id: i64,
) -> Result<Vec<ArticleTags>, AppError> {
    let res = sqlx::query_as::<_, ArticleTags>(
        r#"
        SELECT
            t.id,
            t.tag
        FROM
            tags_table t
        LEFT JOIN
            article_tags_table at ON t.id = at.tag_id
        WHERE
            at.article_id = ?
        "#,
    )
    .bind(article_id)
    .fetch_all(pool)
    .await;

    match res {
        Ok(tags) => {
            debug!("get tags by article_id success");
            Ok(tags)
        }
        Err(e) => {
            error!("get tags by article_id failed: {:?}", e);
            Err(AppError::InternalError)
        }
    }
}
