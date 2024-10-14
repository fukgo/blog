use std::env;
use std::sync::{Arc, Mutex};
use anyhow::Context;
use axum::body::Body;
use axum::extract::Request;
use axum::Router;
use axum::routing::{delete, get, post};
use axum_session::SameSite;
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::AUTHORIZATION;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::{info, error,debug,trace};
use tower_sessions::{Expiry, MemoryStore,SessionManagerLayer};
use blog_backend::model::AppState;
use axum::middleware::from_fn_with_state;
use blog_backend::middleware::require_login;
use blog_backend::handles::{article::*, tag::*, user::*};
use reqwest::header::HeaderValue;
#[tokio::main]
async fn main(){
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    dotenv().ok(); // 加载 .env 文件

    run().await.unwrap();
}


async fn run()->anyhow::Result<()>{

    let database_url = env::var("DATABASE_URL").with_context(||"database not set")?;
    let addr = env::var("BIND_ADDR").with_context(||"bind_address not set")?;
    let timemout = env::var("TIMEOUT").with_context(||"timeout not set")?;
    // 创建 MySQL 连接池
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_same_site(SameSite::None)
        .with_secure(true)
        .with_http_only(true)
        .with_expiry(Expiry::OnInactivity(tower_sessions::cookie::time::Duration::hours(timemout.parse::<i64>().unwrap())));

    //配置cors
    let cors = CorsLayer::new()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"].into_iter().map(|s| s.parse().unwrap()).collect::<Vec<_>>())
        .allow_headers(vec![CONTENT_TYPE, AUTHORIZATION])
        .allow_origin(HeaderValue::from_static("http://localhost:3000"))
        .allow_credentials(true); // 允许凭据;
    let app_state = Arc::new(AppState {
        pool,
        user_vec:Mutex::new(Vec::new()),
    });
    info!("Server is running on: {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    let article_route = Router::new()
        .route("/", post(post_article).layer(from_fn_with_state(app_state.clone(),require_login)))
        .route("/:article_id", post(update_article).layer(from_fn_with_state(app_state.clone(),require_login)))
        .route("/all", get(get_articles_info))
        .route("/detail/:article_id", get(geget_article_detail))
        .route("/:article_id", delete(delete_article).layer(from_fn_with_state(app_state.clone(),require_login)));




    let tag_route = Router::new()
        .route("/:tag_name", get(creata_tag).layer(from_fn_with_state(app_state.clone(),require_login)))
        .route("/delete/:tag_id", delete(delete_tag).layer(from_fn_with_state(app_state.clone(),require_login)))
        .route("/all", get(get_tags))
        .route("/:tag_id/articles", get(get_tag_articles_info));

    let user_route = Router::new()
        .route("/", get(get_users_info))
        .route("/:user_id", get(get_user_by_id))
        .route("/logout", delete(delete_user_logout).layer(from_fn_with_state(app_state.clone(),require_login)))
        .route("/:user_id", delete(delete_user).layer(from_fn_with_state(app_state.clone(),require_login)))
        .route("/:user_id/articles", get(get_user_article));

    let auth_route = Router::new()
        .route("/token", get(auth_user))
        .route("/session", get(is_login));

    let app = Router::new()
        .nest("/tags", tag_route)
        .nest("/users", user_route)
        .nest("/articles", article_route)
        .nest("/auth", auth_route)
        .layer(session_layer)
        .with_state(app_state.clone())
        .layer(cors)
        .layer(TraceLayer::new_for_http()
            .make_span_with(|request: &Request<Body>| {
                tracing::info_span!("http_request", method = %request.method(), uri = %request.uri())
            })
            .on_response(DefaultOnResponse::new()));
    axum::serve(listener,app).await?;
    Ok(())


}
