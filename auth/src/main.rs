use crate::model::*;
use anyhow::{Context, Ok};
use axum::body::Body;
use axum::extract::Request;
use axum::routing::{get, get_service, post};
use axum::Router;
use axum_csrf::{CsrfConfig, CsrfLayer, CsrfToken};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::{debug, error, info, trace};
use tracing_subscriber::{self, fmt::format};
pub mod auth;
pub mod error;
pub mod handle;
pub mod middleware;
pub mod model;
use handle::*;
use reqwest::header::HeaderName;
use reqwest::header::HeaderValue;
use reqwest::{Method, StatusCode};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tracing_subscriber::EnvFilter;
#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::WARN)
    //     .init();
    dotenv().ok(); // 加载 .env 文件
    //// 从环境变量中读取日志级别，默认级别为 INFO
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
    run().await.unwrap();
}

async fn run() -> anyhow::Result<()> {
    let config = CsrfConfig::default();
    let database_url = env::var("DATABASE_URL").with_context(|| "database not set")?;
    let addr = env::var("BIND_ADDR").unwrap_or("0.0.0.0:8001".to_string());

    // let static_files_service = ServeDir::new("static");

    // 创建 MySQL 连接池
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    //配置cors
    let cors = CorsLayer::new()
        .allow_origin(vec![
            HeaderValue::from_static("http://127.0.0.1:8001"),
            HeaderValue::from_static("http://localhost:8001"),
            HeaderValue::from_static("http://127.0.0.1:3000"),
            HeaderValue::from_static("http://localhost:3000"),
        ])
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_credentials(true)
        .allow_headers(vec![
            HeaderName::from_static("authorization"),
            HeaderName::from_static("content-type"),
        ]);
    let app_state = Arc::new(AppState { pool });
    info!("Server is running on: {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    let app = Router::new()

        // 静态文件处理
        // .route("/login",post(login))
        // .route("/register",post(register))
        .route("/auth/login", get(login_form).post(handle_login_form))
        .route("/auth/register", get(register_form).post(handle_register_form))
        .route("/auth/token",get(auth_token))
        .route("/",get(index))
        .layer(cors)
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http()
            .make_span_with(|request: &Request<Body>| {
                tracing::info_span!("http_request", method = %request.method(), uri = %request.uri())
            })
            .on_response(DefaultOnResponse::new()))
        .with_state(app_state.clone());

    axum::serve(listener, app).await?;
    Ok(())
}
