use std::fs::read_to_string;
use std::sync::Arc;
use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::{Form, Json};
use axum::response::{Html, IntoResponse};
use axum_csrf::{CsrfConfig, CsrfLayer, CsrfToken};
use crate::AppState;
use crate::model::*;
use crate::error::Err;
use tracing::{debug,info,error,trace};
use crate::auth::{generate_token,verify_token};
use crate::GLOBAL_PARAMS;
pub async fn login(
    app_state: State<Arc<AppState>>,
    login_data: Json<LoginRequest>,
) ->Result<impl IntoResponse, Err>{
    let user: Option<User> =
        sqlx::query_as::<_, User>("SELECT * FROM auth_user WHERE username = ?")
            .bind(&login_data.username)
            .fetch_optional(&app_state.pool)
            .await
            .map_err(|e| {
                error!("{}", e);
                Err::InternalError
            })?;
    let user = user.ok_or(Err::UserNotFound)?;
    match bcrypt::verify(&login_data.password, &user.password){
        Ok(valid)=>{
            if valid {
                let params = &*GLOBAL_PARAMS;

                let token = generate_token(&user.username,&params.key).await?;

                let res = LoginResponse{
                    token,
                };
                Ok((StatusCode::OK,Json(res)))
            }else {
                Err(Err::UsernamePasswdError)
            }
        },
        Err(e) => {
            // 错误处理逻辑，例如记录错误或返回错误响应
            error!("Error verifying password: {}", e);
            Err(Err::UsernamePasswdError)
        }
    }

}


    pub async fn register(
        app_state: State<Arc<AppState>>,
        register_data: Json<RegisterRequest>,
    ) -> Result<impl IntoResponse, Err> {
        info!("register data: {:?}", register_data);
        // 检查用户名是否已存在
        let user: Option<User> =
            sqlx::query_as::<_, User>("SELECT * FROM auth_user WHERE username = ?")
                .bind(&register_data.username)
                .fetch_optional(& app_state.pool)
                .await
                .map_err(|e| {
                    error!("{}", e);
                    Err::DataBaseError
                })?;

        if user.is_some() {
            return Err(Err::UserExistence);
        }

        // 对密码进行哈希处理
        let password_hash = bcrypt::hash(&register_data.password, 10).unwrap();

        // 插入新用户数据并获取插入的 ID
        let result = sqlx::query!(
        r#"
        INSERT INTO auth_user (username, password, email)
        VALUES (?, ?, ?)
        "#,
        register_data.username,
        password_hash,
        register_data.email
        ).execute(&app_state.pool).await.map_err(|e| {
            error!("{}", e);
            Err::DataBaseError
        })?;

        let res = MsgResponse{
            msg:"register success".to_string()
        };

        Ok((StatusCode::OK,Json(res)))
    }

pub async fn auth_token(
    app_state: State<Arc<AppState>>,
    req: Request<Body>,
)->Result<impl IntoResponse, Err>{
    debug!("receive request: {:?}", req);
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let params = &*GLOBAL_PARAMS;
                let username = verify_token(&token,&params.key,params.timeout).await.map_err(|e| {
                    error!("{:?}", e);
                    Err::TokenInvalid
                })?;
                let user: Option<AuthedUser> = sqlx::query_as::<_,AuthedUser>("SELECT id,username,email FROM auth_user WHERE username = ?")
                    .bind(&username)
                    .fetch_optional(& app_state.pool)
                    .await
                    .map_err(|e|{
                        error!("{}", e);
                            Err::DataBaseError
                    })?;
                match user{
                    Some(user)=>{
                        debug!("get user info success: {:?}", user);
                        return Ok((StatusCode::OK,Json(user)));
                    },
                    _=>{
                        error!("User not found");
                        return Err(Err::TokenInvalid);
                    }
                }
            }
        }
        error!("Invalid Authorization header");
        return Err(Err::TokenInvalid);
    }
    error!("Missing Authorization header");
    Err(Err::TokenInvalid)
}
pub async fn register_form(token: CsrfToken) -> impl IntoResponse {
    let mut html = read_to_string("templates/register.html").unwrap();
    let authenticity_token = token.authenticity_token().unwrap();

    // 将 CSRF 令牌插入到 HTML 中
    html = html.replace("{{ authenticity_token }}", &authenticity_token);

    (token, Html(html)).into_response()
}
pub async fn login_form(token: CsrfToken) -> impl IntoResponse {
    let mut html = read_to_string("templates/login.html").unwrap();
    let authenticity_token = token.authenticity_token().unwrap();

    // 将 CSRF 令牌插入到 HTML 中
    html = html.replace("{{ authenticity_token }}", &authenticity_token);

    (token, Html(html)).into_response()
}
pub async fn handle_login_form(
    app_state: State<Arc<AppState>>,
    token: CsrfToken,
    Form(login_data): Form<LoginKey>,
) -> Result<impl IntoResponse, Err> {
    // 验证 CSRF 令牌
    if token.verify(&login_data.authenticity_token).is_err() {
        return Err(Err::InvalidCsrfToken);
    }

    let user: Option<User> =
        sqlx::query_as::<_, User>("SELECT * FROM auth_user WHERE username = ?")
            .bind(&login_data.username)
            .fetch_optional(&app_state.pool)
            .await
            .map_err(|e| {
                error!("{}", e);
                Err::InternalError
            })?;
    let user = user.ok_or(Err::UserNotFound)?;
    match bcrypt::verify(&login_data.password, &user.password) {
        Ok(valid) => {
            if valid {
                let params = &*GLOBAL_PARAMS;

                let token = generate_token(&user.username, &params.key).await?;

                let res = LoginResponse {
                    token,
                };
                Ok((StatusCode::OK, Json(res)))
            } else {
                Err(Err::UsernamePasswdError)
            }
        },
        Err(e) => {
            error!("Error verifying password: {}", e);
            Err(Err::UsernamePasswdError)
        }
    }
}
pub async fn handle_register_form(
    app_state: State<Arc<AppState>>,
    token: CsrfToken,
    Form(register_data): Form<RegisterKey>,
) -> Result<impl IntoResponse, Err> {
    // 验证 CSRF 令牌
    if token.verify(&register_data.authenticity_token).is_err() {
        return Err(Err::InvalidCsrfToken);
    }

    info!("register data: {:?}", register_data);
    let user: Option<User> =
        sqlx::query_as::<_, User>("SELECT * FROM auth_user WHERE username = ?")
            .bind(&register_data.username)
            .fetch_optional(&app_state.pool)
            .await
            .map_err(|e| {
                error!("{}", e);
                Err::DataBaseError
            })?;

    if user.is_some() {
        return Err(Err::UserExistence);
    }

    let password_hash = bcrypt::hash(&register_data.password, 10).unwrap();

    let result = sqlx::query!(
        r#"
        INSERT INTO auth_user (username, password, email)
        VALUES (?, ?, ?)
        "#,
        register_data.username,
        password_hash,
        register_data.email
    ).execute(&app_state.pool).await.map_err(|e| {
        error!("{}", e);
        Err::DataBaseError
    })?;

    let res = MsgResponse {
        msg: "register success".to_string(),
    };

    Ok((StatusCode::OK, Json(res)))
}