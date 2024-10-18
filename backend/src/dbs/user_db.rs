use crate::model::*;
use crate::utils::get_now_date;
use crate::{error::AppError, model::AuthedUser};
use sqlx::MySqlPool;
use tracing::{debug, error, info};

pub async fn get_user_info_by_auth(
    pool: &MySqlPool,
    user: &AuthedUser,
)-> Result<AuthedUser, &'static str> {
    let user_res = sqlx::query_as::<_, AuthedUser>(
        r#"SELECT id, username, email,nickname FROM user_table WHERE username = ?"#,
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

pub async fn storage_auth_user(pool: &MySqlPool, user: &AuthedUser) -> Result<(), AppError> {
    let res = sqlx::query(r#"INSERT INTO user_table (username,email) VALUES (?, ?)"#)
        .bind(&user.username)
        .bind(&user.email)
        .execute(pool)
        .await;

    match res {
        Ok(_) => {
            debug!("storage user info success");
            Ok(())
        }
        Err(e) => {
            error!("storage user info failed: {:?}", e);
            Err(AppError::DataBaseError)
        }
    }
}

pub async fn get_users_info_db(
    pool:&MySqlPool,
)->Result<Vec<User>,AppError>{
    let users = sqlx::query_as::<_, User>(
        r#"SELECT * FROM user_table"#,
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


pub async fn get_user_by_id_db(
    pool:&MySqlPool,
    user_id: i64,
)->Result<User,AppError>{


    let user = sqlx::query_as::<_, User>(
        r#"SELECT * FROM user_table WHERE id = ?"#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await;

    match user {
        Ok(user) => {
            debug!("get user info success");
            Ok(user)
        }
        Err(e) => {
            error!("get user info failed: {:?}", e);
            Err(AppError::UserNotFound)
        }
    }
}

pub async fn get_user_by_username_db(
    pool:&MySqlPool,
    username: &str,
)->Result<User,AppError>{

    let user = sqlx::query_as::<_, User>(
        r#"SELECT * FROM user_table WHERE username = ?"#,
    )
    .bind(username)
    .fetch_one(pool)
    .await;

    match user {
        Ok(user) => {
            debug!("get user info success");
            Ok(user)
        }
        Err(e) => {
            error!("get user info failed: {:?}", e);
            Err(AppError::UserNotFound)
        }
    }
}
pub async fn delete_user_db(
    pool:&MySqlPool,
    user_id: i64,
)->Result<(),AppError>{
    let res = sqlx::query(r#"DELETE FROM user_table WHERE id = ?"#)
        .bind(user_id)
        .execute(pool)
        .await;

    match res {
        Ok(_) => {
            debug!("delete user success");
            Ok(())
        }
        Err(e) => {
            error!("delete user failed: {:?}", e);
            Err(AppError::UserNotFound)
        }
    }
}

pub async fn get_resume_by_userid_db(
    pool:&MySqlPool,
    user_id: i64,
)->Result<Resume,AppError>{

    let resumes = sqlx::query_as::<_, Resume>(
        r#"SELECT * FROM resume_table WHERE user_id = ?"#,
    )
    .bind(user_id)
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
    user_id: i64,
) -> Result<(), AppError> {
    // 检查简历是否存在
    let existing_resume = sqlx::query(r#"SELECT id FROM resume_table WHERE user_id = ?"#)
        .bind(user_id)
        .fetch_optional(pool)
        .await;

    match existing_resume {
        Ok(Some(_)) => {
            // 如果存在，则更新简历
            let res = sqlx::query(r#"UPDATE resume_table SET content = ? WHERE user_id = ?"#)
                .bind(&resume.content)
                .bind(user_id)
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
            let res = sqlx::query(r#"INSERT INTO resume_table (user_id, content) VALUES (?, ?)"#)
                .bind(user_id)
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
pub async fn update_user_db(
    pool: &MySqlPool,
    user: &UserUpdate,
    user_id:i64
)->Result<(), AppError>{
    let mut query = "UPDATE user_table SET ".to_string();
    let mut updates = Vec::new();
    if let Some(ref nickname) = user.nickname {
        updates.push("nickname = ?".to_string());
    }
    //todo!
    // 检查是否有任何字段需要更新
    if updates.is_empty() {
        debug!("No fields to update for article_id: {}", user_id);
        return Ok(()); // 没有字段需要更新，直接返回成功
    }
    query += &updates.join(", "); // 以逗号连接各个更新片段
    query += " WHERE id = ?"; // 添加条件
    let mut sql_query = sqlx::query(&query); // 使用不同的变量名
    if let Some(ref nickname) = user.nickname {
        sql_query = sql_query.bind(nickname);
    }
    sql_query = sql_query.bind(user_id);
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

