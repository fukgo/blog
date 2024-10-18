use axum::{
    response::IntoResponse,
    http::StatusCode,
    Router,
};
use axum::Json;
use tower_sessions::Session;
use tracing::{info,debug,error};
use crate::model::*;
use crate::error::*;
use core::error;
use std::sync::Arc;
use axum::extract::State;
use axum::extract::Request;
use axum::body::Body;
use axum::extract::Path;
use crate::utils::get_auth;
use crate::dbs::user_db::*;
use crate::dbs::user_db::*;
use axum::extract::Query;



// // 更新当前用户信息
// pub async fn put_users(
//     app_state: State<Arc<AppState>>,

// ) -> Result<impl IntoResponse, AppError> {
    
//     Ok(StatusCode::OK)
// }
// 删除指定用户
pub async fn delete_user(
    app_state: State<Arc<AppState>>,
    Path(user_id): Path<i64>
) -> Result<impl IntoResponse, AppError> {
    let _ = delete_user_db(&app_state.pool,user_id).await?;
    Ok(StatusCode::OK)
}
// 用户退出登录
pub async fn delete_user_logout(
    app_state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
   let user_get = session.get::<User>("user").await.unwrap().unwrap();
    session.clear().await;
    {
        let mut vec = app_state.user_vec.lock().unwrap();
        if let Some(pos) = vec.iter().position(|user| user.username == user_get.username) {
            vec.remove(pos);
        }
    }
    Ok(StatusCode::OK)
}
// 获取所有用户信息
pub async fn get_users_info(
    app_state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let users = get_users_info_db(&app_state.pool).await?;
    Ok((StatusCode::OK, Json(users)))
}
// 根据用户ID获取用户信息
pub async fn get_user_by_id(
    app_state: State<Arc<AppState>>,
    Path(user_id): Path<i64>
) -> Result<impl IntoResponse, AppError> {
    let user = get_user_by_id_db(&app_state.pool,user_id).await?;
    Ok((StatusCode::OK, Json(user)))

}


pub async fn auth_user(
    app_state: State<Arc<AppState>>,
    session: Session,
    req: Request<Body>,
)->Result<impl IntoResponse,AppError>{
    //从header 获取token
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                debug!("token: {}", token);

                let auth_token_url = std::env::var("AUTH_TOKEN_URL").map_err(|e|{
                    error!("missed auth_token_url");
                    AppError::InternalError
                })?;
                let user = get_auth(token, &auth_token_url).await?;
                // cookies.add(Cookie::new("user", user.username.clone()));
                let res = session.insert("user", &user.username).await.map_err(|e|{
                    error!("session insert error: {:?}", e);
                    AppError::InternalError
                })?;
                {
                    let mut vec = app_state.user_vec.lock().unwrap();
                    vec.push(user.clone());
                }
                if let Err(e) = get_user_by_username_db(&app_state.pool, &user.username).await{
                    debug!("user not found, storage user info");
                    storage_auth_user(&app_state.pool, &user).await?;
                }

                return Ok((StatusCode::OK,Json(user)));
            }
        }
        error!("token invalid");
        return Err(AppError::TokenInvalid);
    }
    error!("token not found");
    Err(AppError::TokenInvalid)
}

pub async fn is_login(
    app_state: State<Arc<AppState>>,
    session: Session
)->Result<impl IntoResponse,AppError>{
    let user_username_option = session.get::<String>("user").await.map_err({
        |e| {
            error!("session get error: {:?}", e);
            AppError::InternalError
        }
    })?;
    if let Some(user_username) = user_username_option {
        let user = get_user_by_username_db(&app_state.pool, &user_username).await?;
        return Ok((StatusCode::OK,Json(user)));
    }else {
        return Err(AppError::UserUnLogin);
    }
}

pub async fn get_user_resume(
    app_state: State<Arc<AppState>>,
    session: Session,
    Path(user_id): Path<i64>
)->Result<impl IntoResponse,AppError>{
    // let user_username_option = session.get::<String>("user").await.map_err({
    //     |e| {
    //         error!("session get error: {:?}", e);
    //         AppError::InternalError
    //     }
    // })?;
    let resume = get_resume_by_userid_db(&app_state.pool, user_id).await?;
    Ok((StatusCode::OK,Json(resume)))
}
#[axum::debug_handler]
pub async fn post_resume(
    app_state: State<Arc<AppState>>,
    session: Session,
    Path(user_id): Path<i64>,
    Json(resume): Json<ResumeCreate>,


)->Result<impl IntoResponse,AppError>{
    let _ = save_or_update_resume_db(&app_state.pool, &resume,user_id).await?;
    Ok(StatusCode::OK)
}



pub async fn update_user(
    app_state: State<Arc<AppState>>,
    Path(user_id): Path<i64>,
    Json(user_update): Json<UserUpdate>,
)->Result<impl IntoResponse,AppError>{
    update_user_db(&app_state.pool,&user_update,user_id).await?;
    Ok(())
}