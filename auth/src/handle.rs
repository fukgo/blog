use crate::auth::{generate_token, verify_token};
use crate::error::Err;
use crate::model::*;
use crate::AppState;
use ninja::Ninja;
use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::response::{Html, IntoResponse};
use axum::{response::Redirect, routing::post, Router};
use axum::{Form, Json};
use axum_csrf::{CsrfConfig, CsrfLayer, CsrfToken};
use std::fs::read_to_string;
use std::sync::Arc;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tracing::{debug, error, info, trace};
use uuid::Uuid;
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
                let key = std::env::var("KEY").unwrap_or_else(|_| "keynotset".to_string());

                let token = generate_token(&user.username, &key).await?;

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
                let key = std::env::var("KEY").unwrap_or_else(|_| "keynotset".to_string());
                let timeout = std::env::var("TOKEN_TIMEOUT")
                    .unwrap_or_else(|_| "2".to_string())
                    .parse::<u64>()
                    .unwrap();
                let username = verify_token(&token, &key, timeout).await.map_err(|e| {
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
                    let key = std::env::var("KEY").unwrap_or_else(|_| "keynotset".to_string());
                    let token = generate_token(&user.username, &key).await?;

                    if let Some(redirect) = login_data.redirect {
                        let redirect_url = if redirect.starts_with("http://")
                            || redirect.starts_with("https://")
                        {
                            redirect.to_string()
                        } else {
                            format!("https://{}", redirect) // 默认使用 https://，你也可以改为 http://
                        };
                        // 跳转到指定页面
                        debug!("redirect to: {}", redirect_url);
                        Ok(Redirect::to(&format!("{}?token={}", redirect_url, token)))
                    } else {
                        let res = LoginResponse { token };
                        debug!("login success: {:?}", res);
                        return Ok(Redirect::to("/")); 
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
            if register_data.access != "qweasdzxc" {
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

use serde_json::Value; // 引入 serde_json 库

use tera::{Tera, Context};

pub async fn index(app_state: State<Arc<AppState>>) -> Result<impl IntoResponse, Err> {
    let rows = sqlx::query_as::<_, (i32, String, Value)>(r#"
        SELECT
            sc.id,
            sc.category_name,
            JSON_ARRAYAGG(
                JSON_OBJECT(
                    'site_title', sl.site_title,
                    'site_info', sl.site_info,
                    'site_url', sl.site_url
                )
            ) AS web_list
        FROM
            site_catalogues sc
        LEFT JOIN
            site_list sl ON sc.id = sl.catalogue_id
        GROUP BY
            sc.id, sc.category_name;
    "#)
    .fetch_all(&app_state.pool)
    .await
    .map_err(|e| {
        error!("{}", e);
        Err::DataBaseError
    })?;

    let mut site_vec: Vec<SiteVec> = Vec::new();

    // Parse JSON results
    for (id, category_name, web_list_json) in rows {
        let web_list: Vec<SiteInfo> = serde_json::from_value(web_list_json).unwrap_or_default();
        site_vec.push(SiteVec {
            id,
            category_name,
            web_list,
        });
    }

    // 初始化 Tera
    let tera = Tera::new("templates/**/*").unwrap();
    
    // 创建上下文并插入数据
    let mut context = Context::new();
    context.insert("categories", &site_vec);

    // 渲染模板
    let rendered = tera.render("index.html", &context).map_err(|e| {
        error!("{}", e);
        Err::TemplateError
    })?;

    // 返回渲染的 HTML
    Ok(Html(rendered))
}
