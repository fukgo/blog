use std::env;
use std::sync::Arc;
use anyhow::{Context, Ok};
use axum::body::Body;
use axum::extract::Request;
use axum::Router;
use axum::routing::{get, get_service, post};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing_subscriber::{self, fmt::format};
use tower_http::services::ServeDir;
use tracing::{info, error,debug,trace};
use lazy_static::lazy_static;
use crate::model::*;
use axum_csrf::{CsrfConfig, CsrfLayer, CsrfToken};
pub mod auth;
pub mod handle;
pub mod error;
pub mod model;
pub mod middleware;
use reqwest::StatusCode;
use handle::*;
lazy_static! {
    pub static ref GLOBAL_PARAMS: Params= {
        dotenv().ok(); // 加载 .env 文件
        let secret_key: String = std::fs::read_to_string("key").expect("Failed to read the key file");
        let timeout_hour = env::var("TOKEN_TIMEOUT").unwrap_or_else(|_| "2".to_string());
        let timout_seconds = timeout_hour.parse::<u64>().with_context(|| "timeout set error").expect("timeout invalid") * 3600;
        Params{
            key:secret_key,
            timeout:timout_seconds
        }
    };
}

#[tokio::main]
async fn main(){
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    dotenv().ok(); // 加载 .env 文件

    run().await.unwrap();
}


async fn run()->anyhow::Result<()>{
    let config = CsrfConfig::default();
    let database_url = env::var("DATABASE_URL").with_context(||"database not set")?;
    let addr = env::var("BIND_ADDR").with_context(||"bind_address not set")?;

    // let static_files_service = ServeDir::new("static");

    // 创建 MySQL 连接池
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    //配置cors
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);
    let app_state = Arc::new(AppState {
        pool,
    });
    info!("Server is running on: {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    let app = Router::new()

        // 静态文件处理
        // .route("/login",post(login))
        // .route("/register",post(register))
        .route("/auth/login", get(login_form).post(handle_login_form))
        .route("/auth/register", get(register_form).post(handle_register_form))
        .route("/",get(auth_token))

        .layer(CsrfLayer::new(config))
        .layer(cors)
        .layer(TraceLayer::new_for_http()
            .make_span_with(|request: &Request<Body>| {
                tracing::info_span!("http_request", method = %request.method(), uri = %request.uri())
            })
            .on_response(DefaultOnResponse::new()))
        .with_state(app_state.clone());
    axum::serve(listener,app).await?;
    Ok(())


}


