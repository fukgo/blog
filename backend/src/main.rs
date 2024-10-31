use anyhow::Context;
use axum::body::Body;
use axum::extract::Request;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, post};
use axum::Router;
use axum_session::SameSite;
use backend::handles::{article::*, tag::*, user::*};
use backend::handles::{catalogue::*, comment::*};
use backend::middleware::require_login;
use backend::models::state::AppState;
use dotenv::dotenv;
use reqwest::header::HeaderValue;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tracing::{debug, error, info, trace};
#[tokio::main]
async fn main() {
    //// 从环境变量中读取日志级别，默认级别为 INFO
    // let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // tracing_subscriber::fmt().with_env_filter(env_filter).init();
    // env RUST_LOG=debug
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    dotenv().ok(); // 加载 .env 文件

    run().await.unwrap();
}

async fn run() -> anyhow::Result<()> {
    let database_url = env::var("DATABASE_URL").with_context(|| "database not set")?;
    let addr = env::var("BIND_ADDR").unwrap_or("0.0.0.0:8002".to_string());
    let timemout = env::var("TIMEOUT").unwrap_or("2".to_string());
    // 创建 MySQL 连接池
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_same_site(SameSite::None) // 在生产环境中应保持为 true，以确保 Cookie 只能通过 HTTPS 发送
        .with_secure(true)
        .with_http_only(true)
        .with_expiry(Expiry::OnInactivity(
            tower_sessions::cookie::time::Duration::hours(timemout.parse::<i64>()?),
        ));

    //配置cors
    let cors = CorsLayer::new()
        .allow_methods(
            vec!["GET", "POST", "PUT", "DELETE"]
                .into_iter()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>(),
        )
        .allow_headers(vec![CONTENT_TYPE, AUTHORIZATION])
        .allow_origin(vec![
            HeaderValue::from_static("http://localhost:3000"),
            HeaderValue::from_static("http://127.0.0.1:3000"),
            HeaderValue::from_static("http://127.0.0.1:8001"),
            HeaderValue::from_static("http://localhost:8001"),
            HeaderValue::from_static("http://127.0.0.1:5500"),
        ])
        .allow_credentials(true); // 允许凭据;
    let app_state = Arc::new(AppState {
        pool,
        user_vec: Mutex::new(Vec::new()),
    });
    info!("Server is running on: {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    let article_route = Router::new()
        .route("/", post(post_article).layer(from_fn_with_state(app_state.clone(),require_login)))
        .route("/:article_id", post(update_article).layer(from_fn_with_state(app_state.clone(),require_login)))
        .route("/feature", get(get_featured_article))
        .route("/detail/:article_id", get(get_article_detail))
        .route("/late", get(get_articles_latest))
        .route("/:article_id", delete(delete_article).layer(from_fn_with_state(app_state.clone(),require_login)))
        .route("/titles/all", get(get_article_titles))
        .route("/:article_id/tags", get(get_article_tags));

    let tag_route = Router::new()
        .route("/:tag_name", get(creata_tag))
        .route("/delete/:tag_id", delete(delete_tag).layer(from_fn_with_state(app_state.clone(),require_login)))
        .route("/all", get(get_tags))
        .route("/:tag_id/articles", get(get_tag_articles_info));

    let user_route = Router::new()
        .route("/", get(get_users_info))
        .route("/:user_detail_id", get(get_user_by_id))
        .route("/:user_detail_id/update", post(update_user).layer(from_fn_with_state(app_state.clone(),require_login)))
        .route("/logout", delete(delete_user_logout).layer(from_fn_with_state(app_state.clone(),require_login)))
        .route("/:user_id/articles", get(get_user_article))
        .route("/:user_id/resume", get(get_user_resume))
        .route(
            "/:user_id/resume",
            //更新或创建简历
            post(post_resume),
        );

    let auth_route = Router::new()
        .route("/token", get(auth_user))
        .route("/session", get(is_login));
    let comment_route = Router::new()
        .route("/post", post(post_comment).layer(from_fn_with_state(app_state.clone(),require_login)))
        .route("/:article_id", get(get_comments_by_article_id));
    let catalogue_route = Router::new()
        .route("/", post(post_catalogue).layer(from_fn_with_state(app_state.clone(),require_login)))
        .route("/:catalogue_id", post(post_update_catalogue).layer(from_fn_with_state(app_state.clone(),require_login)))
        .route("/:catalogue_id", delete(delete_catalogue).layer(from_fn_with_state(app_state.clone(),require_login)))
        .route("/all", get(get_all_catalogues))
        .route("/:catalogue_id", get(get_catalogue_by_id))
        .route("/:catalogue_id/articles", get(get_catalogue_article_titles))
        //移除目录下的文章
        .route(
            "/delete/:catalogue_id/:article_id",
            delete(delete_catalogue_article_by_id),
        )
        .route(
            "/delete/:catalogue_id/all",
            delete(delete_catalogue_all_articles),
        )
        //添加文章到目录
        .route("/add", post(post_catalogue_article))
        .route("/post/sorder", post(post_catalogue_article_sort));

    let app = Router::new()
        .nest("/api/tags", tag_route)
        .nest("/api/users", user_route)
        .nest("/api/articles", article_route)
        .nest("/api/auth", auth_route)
        .nest("/api/comments", comment_route)
        .nest("/api/catalogues", catalogue_route)
        .layer(session_layer)
        .with_state(app_state.clone())
        .layer(cors)
        .layer(TraceLayer::new_for_http()
            .make_span_with(|request: &Request<Body>| {
                tracing::info_span!("http_request", method = %request.method(), uri = %request.uri())
            })
            .on_response(DefaultOnResponse::new()));
    axum::serve(listener, app).await?;
    Ok(())
}
