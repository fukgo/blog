use crate::auth::{generate_token, verify_token};
use crate::error::Err;
use crate::model::*;
use crate::AppState;
use crate::GLOBAL_PARAMS;
use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::response::{Html, IntoResponse};
use axum::{Form, Json};
use axum_csrf::{CsrfConfig, CsrfLayer, CsrfToken};
use std::fs::read_to_string;
use std::sync::Arc;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tracing::{debug, error, info, trace};
use uuid::Uuid;
use axum::{
    routing::post,
    response::Redirect,
    Router,
};
pub async fn login(
    app_state: State<Arc<AppState>>,
    login_data: Json<LoginRequest>,
) -> Result<impl IntoResponse, Err> {
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

                let res = LoginResponse { token };
                Ok((StatusCode::OK, Json(res)))
            } else {
                Err(Err::UsernamePasswdError)
            }
        }
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
            .fetch_optional(&app_state.pool)
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
    )
    .execute(&app_state.pool)
    .await
    .map_err(|e| {
        error!("{}", e);
        Err::DataBaseError
    })?;

    let res = MsgResponse {
        msg: "register success".to_string(),
    };

    Ok((StatusCode::OK, Json(res)))
}

pub async fn auth_token(
    app_state: State<Arc<AppState>>,
    req: Request<Body>,
) -> Result<impl IntoResponse, Err> {
    debug!("receive request: {:?}", req);
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let params = &*GLOBAL_PARAMS;
                let username = verify_token(&token, &params.key, params.timeout)
                    .await
                    .map_err(|e| {
                        error!("{:?}", e);
                        Err::TokenInvalid
                    })?;
                let user: Option<AuthedUser> = sqlx::query_as::<_, AuthedUser>(
                    "SELECT id,username,email FROM auth_user WHERE username = ?",
                )
                .bind(&username)
                .fetch_optional(&app_state.pool)
                .await
                .map_err(|e| {
                    error!("{}", e);
                    Err::DataBaseError
                })?;
                match user {
                    Some(user) => {
                        debug!("get user info success: {:?}", user);
                        return Ok((StatusCode::OK, Json(user)));
                    }
                    _ => {
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

pub async fn register_form(cookies: Cookies) -> impl IntoResponse {
    let mut html = read_to_string("templates/register.html").unwrap();
    let csrf_token = uuid::Uuid::new_v4().to_string();
    cookies.add(Cookie::new("csrf_token", csrf_token.clone()));

    // 将 CSRF 令牌插入到 HTML 中
    html = html.replace("{{ authenticity_token }}", &csrf_token);

    Html(html)
}
pub async fn login_form(cookies: Cookies) -> impl IntoResponse {
    let mut html = read_to_string("templates/login.html").unwrap();
    let csrf_token = uuid::Uuid::new_v4().to_string();
    cookies.add(Cookie::new("csrf_token", csrf_token.clone()));
    // debug!("authenticity_token: {}，{}", authenticity_token,token.authenticity_token().unwrap());
    // 将 CSRF 令牌插入到 HTML 中
    html = html.replace("{{ authenticity_token }}", &csrf_token);

    Html(html)


}
pub async fn handle_login_form(
    app_state: State<Arc<AppState>>,
    cookies: Cookies,
    Form(login_data): Form<LoginKey>,
) -> Result<impl IntoResponse, Err> {
    // 验证 CSRF 令牌
    if let Some(csrf_token) = cookies.get("csrf_token") {
        if csrf_token.value() != login_data.authenticity_token {
            error!(
                "Invalid CSRF token: submitted {}, expected {}",
                login_data.authenticity_token,
                csrf_token.value()
            );
            return Err(Err::InvalidCsrfToken);
        }

        let user: Option<User> = sqlx::query_as::<_, User>("SELECT * FROM auth_user WHERE username = ?")
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

                    if let Some(redirect) = login_data.redirect {
                        let redirect_url = if redirect.starts_with("http://") || redirect.starts_with("https://") {
                            redirect.to_string()
                        } else {
                            format!("https://{}", redirect)  // 默认使用 https://，你也可以改为 http://
                        };
                        // 跳转到指定页面
                        Ok(Redirect::to(&format!("{}?token={}", redirect_url, token)))
                    } else {
                        let res = LoginResponse { token };
                        return Ok(Redirect::to("/")); // 直接返回 JSON 响应
                    }
                } else {
                    error!("Invalid username or password");
                    return Err(Err::UsernamePasswdError);
                }
            }
            Err(e) => {
                error!("Error verifying password: {}", e);
                return Err(Err::UsernamePasswdError);
            }
        }
    } else {
        error!("CSRF token not found in cookies");
        return Err(Err::InvalidCsrfToken);
    }
}
pub async fn handle_register_form(
    app_state: State<Arc<AppState>>,
    cookies: Cookies,
    Form(register_data): Form<RegisterKey>,
) -> Result<impl IntoResponse, Err> {
    // 验证 CSRF 令牌
    if let Some(csrf_token) = cookies.get("csrf_token") {
        if csrf_token.value() != register_data.authenticity_token {
            error!(
                "Invalid CSRF token: submitted {}, expected {}",
                register_data.authenticity_token,
                csrf_token.value()
            );
            return Err(Err::InvalidCsrfToken);
        } else {
            if register_data.access != "qweasdzxc"{
                error!("accedd invalid");
                return Err(Err::AccessError);
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
                error!("User already exists");
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
            )
            .execute(&app_state.pool)
            .await
            .map_err(|e| {
                error!("{}", e);
                Err::DataBaseError
            })?;

            let res = MsgResponse {
                msg: "register success".to_string(),
            };

            Ok((StatusCode::OK, Json(res)))
        }
    } else {
        error!("CSRF token not found in cookies");
        return Err(Err::InvalidCsrfToken);
    }
}
