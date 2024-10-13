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


