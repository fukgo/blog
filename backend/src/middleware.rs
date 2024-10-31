use crate::error::AppError;
use crate::models::state::AppState;
use axum::body::Body;
use axum::extract::State;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use std::sync::Arc;
use tower_sessions::Session;
use tracing::{error, info};
// 定义一个中间件，验证用户是否登录
pub async fn require_login(
    State(state): State<Arc<AppState>>,
    req: Request<Body>, // 泛型 B 处理请求体类型
    next: Next,         // 链中的下一个中间件或处理器
) -> Result<impl IntoResponse, AppError> {
    // 提取 Session
    if let Some(session) = req.extensions().get::<Session>() {
        // 判断 session 中是否存在 user_id
        if let Some(user_id) = session
            .get::<String>("user")
            .await
            .map_err(|e| AppError::InternalError)?
        {
            info!("User ID: {} is logged in", user_id);
            // 用户已登录，继续处理请求
            Ok(next.run(req).await)
        } else {
            // 未登录，返回未授权
            Err(AppError::UserUnLogin)
        }
    } else {
        // 未找到 Session，返回未授权
        Err(AppError::UserUnLogin)
    }
}
