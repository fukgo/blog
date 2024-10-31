use crate::models::user::*;
use crate::{error::AppError, models::user::*};
use sqlx::MySqlPool;
use sqlx::Row;
use tracing::{debug, error, info};
pub async fn get_user_info_by_auth(
    pool: &MySqlPool,
    user: &User,
) -> Result<UserDetailDisplay, &'static str> {
    let user_res = sqlx::query_as::<_, UserDetailDisplay>(
        r#"select
    u.id,
    u.email,
    u.username,
    d.avatar,
    d.nickname,
    d.skills
    from user_table as u join user_detail_table as d on u.id = d.user_id where u.username=?;"#,
    )
    .bind(&user.username)
    .fetch_one(pool)
    .await;

    match user_res {
        Ok(user) => {
            if user.username.is_empty() {
                error!("username field is empty");
                Err("user not found")
            } else {
                debug!("get user info success");
                Ok(user)
            }
        }
        Err(e) => {
            error!("get user info failed: {:?}", e);
            Err("user not found")
        }
    }
}
pub async fn storage_auth_user(pool: &MySqlPool, user: &User) -> Result<(), AppError> {
    // 执行插入 user_table，并返回插入的 ID
    let _user_res = sqlx::query(
        r#"
        INSERT INTO user_table (username, email) VALUES (?,?)
        "#,
    )
    .bind(&user.username)
    .bind(&user.email)
    .execute(pool) // 使用 execute 只执行插入
    .await
    .map_err(|e| {
        error!("storage user info failed: {:?}", e);
        AppError::DataBaseError
    })?;

    // 获取插入的 ID
    let user_id: i64 = sqlx::query_scalar("SELECT id FROM user_table WHERE username = ?")
        .bind(&user.username)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            error!("storage user info failed: {:?}", e);
            AppError::DataBaseError
        })?;
    debug!("user_id: {}", user_id);
    debug!("storage user info success");

    // 插入 user_detail_table，使用获取的 ID
    sqlx::query(r#"INSERT INTO user_detail_table (user_id, nickname) VALUES (?, ?)"#)
        .bind(user_id) // 使用插入的 ID
        .bind(&user.username) // 假设 nickname 使用 username
        .execute(pool)
        .await
        .map_err(|e| {
            error!("storage user detail info failed: {:?}", e);
            AppError::DataBaseError
        })?;

    Ok(())
}

pub async fn get_users_info_db(pool: &MySqlPool) -> Result<Vec<UserInfo>, AppError> {
    let users = sqlx::query_as::<_, UserInfo>(
        r#"
select id,nickname,avatar from user_detail_table;
    "#,
    )
    .fetch_all(pool)
    .await;

    match users {
        Ok(users) => {
            debug!("get users info success");
            Ok(users)
        }
        Err(e) => {
            error!("get users info failed: {:?}", e);
            Err(AppError::UserNotFound)
        }
    }
}

pub async fn get_users_info_by_id_db(pool: &MySqlPool, user_id: i64) -> Result<UserInfo, AppError> {
    let user = sqlx::query_as::<_, UserInfo>(
        r#"select id,nickname,avatar from user_detail_table where id=?;"#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        error!("get user info failed: {:?}", e);
        AppError::UserNotFound
    })?;
    Ok(user)
}

pub async fn get_user_detail_by_id_db(pool: &MySqlPool, id: i64) -> Result<UserDetail, AppError> {
    let user = sqlx::query_as::<_, UserDetail>(
        r#"select * from user_detail_table where id=?;
"#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        error!("get user info failed: {:?}", e);
        AppError::UserNotFound
    })?;

    Ok(user)
}

pub async fn get_user_by_username_db(pool: &MySqlPool, username: &str) -> Result<User, AppError> {
    let user = sqlx::query_as::<_, User>(r#"SELECT * FROM user_table WHERE username = ?"#)
        .bind(username)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            error!("get user info failed: {:?}", e);
            AppError::UserNotFound
        })?;

    match user {
        Some(user) => {
            debug!("get user info success");
            Ok(user)
        }
        _ => {
            error!("get user info failed");
            Err(AppError::UserNotFound)
        }
    }
}

pub async fn get_session_user_by_username_db(
    pool: &MySqlPool,
    username: &str,
) -> Result<UserSession, AppError> {
    let user = sqlx::query_as::<_, UserSession>(
        r#"
        SELECT 
            b.id AS user_detail_id, 
            u.username AS username, 
            u.email AS email, 
            b.nickname AS nickname, 
            b.avatar AS avatar
        FROM 
            user_table u
        JOIN 
            user_detail_table b 
        ON 
            u.id = b.user_id
        WHERE 
            u.username = ?;
        "#,
    )
    .bind(username)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        error!("get user info failed: {:?}", e);
        AppError::UserNotFound
    })?;
    if let Some(user) = user {
        debug!("get user info success");
        Ok(user)
    } else {
        error!("get user info failed");
        Err(AppError::UserNotFound)
    }
}

pub async fn get_resume_by_userid_db(
    pool: &MySqlPool,
    user_detail_id: i64,
) -> Result<Resume, AppError> {
    let resumes =
        sqlx::query_as::<_, Resume>(r#"SELECT * FROM resume_table WHERE user_detail_id = ?"#)
            .bind(user_detail_id)
            .fetch_one(pool)
            .await;

    match resumes {
        Ok(resumes) => {
            debug!("get resumes info success");
            Ok(resumes)
        }
        Err(e) => {
            error!("get resumes info failed: {:?}", e);
            Err(AppError::UserNotFound)
        }
    }
}

pub async fn save_or_update_resume_db(
    pool: &MySqlPool,
    resume: &ResumeCreate,
    user_detail_id: i64,
) -> Result<(), AppError> {
    // 检查简历是否存在
    let existing_resume = sqlx::query(r#"SELECT id FROM resume_table WHERE user_detail_id = ?"#)
        .bind(user_detail_id)
        .fetch_optional(pool)
        .await;

    match existing_resume {
        Ok(Some(_)) => {
            // 如果存在，则更新简历
            let res =
                sqlx::query(r#"UPDATE resume_table SET content = ? WHERE user_detail_id = ?"#)
                    .bind(&resume.content)
                    .bind(user_detail_id)
                    .execute(pool)
                    .await;

            match res {
                Ok(_) => {
                    debug!("update resume success");
                    Ok(())
                }
                Err(e) => {
                    error!("update resume failed: {:?}", e);
                    Err(AppError::UserNotFound)
                }
            }
        }
        Ok(None) => {
            // 如果不存在，则插入简历
            let res =
                sqlx::query(r#"INSERT INTO resume_table (user_detail_id, content) VALUES (?, ?)"#)
                    .bind(user_detail_id)
                    .bind(&resume.content)
                    .execute(pool)
                    .await;

            match res {
                Ok(_) => {
                    debug!("post resume success");
                    Ok(())
                }
                Err(e) => {
                    error!("post resume failed: {:?}", e);
                    Err(AppError::UserNotFound)
                }
            }
        }
        Err(e) => {
            error!("failed to check if resume exists: {:?}", e);
            Err(AppError::UserNotFound)
        }
    }
}

pub async fn update_userdetail_db(
    pool: &MySqlPool,
    user: &UserDetailUpdate,
    user_detail_id: i64,
) -> Result<(), AppError> {
    let mut query = "UPDATE user_detail_table SET ".to_string();
    let mut updates = Vec::new();
    if let Some(ref nickname) = user.nickname {
        updates.push("nickname = ?".to_string());
    }
    if let Some(ref avatar) = user.avatar {
        updates.push("avatar = ?".to_string());
    }
    if let Some(ref skills) = user.skills {
        updates.push("skills = ?".to_string());
    }
    if let Some(ref bio) = user.bio {
        updates.push("bio = ?".to_string());
    }
    // 检查是否有任何字段需要更新
    if updates.is_empty() {
        debug!("No fields to update for article_id: {}", user_detail_id);
        return Ok(()); // 没有字段需要更新，直接返回成功
    }
    query += &updates.join(", "); // 以逗号连接各个更新片段
    query += " WHERE id = ?"; // 添加条件
    let mut sql_query = sqlx::query(&query); // 使用不同的变量名
    if let Some(ref nickname) = user.nickname {
        sql_query = sql_query.bind(nickname);
    }
    if let Some(ref avatar) = user.avatar {
        sql_query = sql_query.bind(avatar);
    }
    if let Some(ref skills) = user.skills {
        sql_query = sql_query.bind(skills);
    }
    if let Some(ref bio) = user.bio {
        sql_query = sql_query.bind(bio);
    }
    sql_query = sql_query.bind(user_detail_id);
    let res = sql_query.execute(pool).await;

    match res {
        Ok(_) => {
            debug!("user update success");
            Ok(())
        }
        Err(e) => {
            error!("update article failed: {:?}", e);
            Err(AppError::InternalError)
        }
    }
}
