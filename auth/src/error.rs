use axum::{
    response::{IntoResponse, Response},
    Json,
    http::StatusCode,
};
use serde::Serialize;

// 定义错误枚举
#[derive(Debug, Serialize)]
pub enum Err {
    UserNotFound,
    UserExistence,
    UsernameExistence,
    UsernamePasswdError,
    EmailExistence,
    InternalError,
    TokenInvalid,
    TokenExpired,
    RequestNotFound,
    DataBaseError,
    InvalidCsrfToken
}

// 为每个错误提供状态码和消息
impl Err {
    fn status_code(&self) -> StatusCode {
        match self {
            Err::UserNotFound => StatusCode::NOT_FOUND,
            Err::UserExistence => StatusCode::CONFLICT,
            Err::UsernameExistence => StatusCode::CONFLICT,
            Err::UsernamePasswdError => StatusCode::CONFLICT,
            Err::EmailExistence => StatusCode::CONFLICT,
            Err::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Err::TokenInvalid => StatusCode::FORBIDDEN,
            Err::TokenExpired => StatusCode::UNAUTHORIZED,
            Err::RequestNotFound => StatusCode::NOT_FOUND,
            Err::DataBaseError => StatusCode::INTERNAL_SERVER_ERROR,
            Err::InvalidCsrfToken => StatusCode::FORBIDDEN,
        }
    }

    fn error_message(&self) -> String {
        match self {
            Err::UserNotFound => "User not found".to_string(),
            Err::UserExistence => "User already exists".to_string(),
            Err::UsernameExistence => "Username already exists".to_string(),
            Err::UsernamePasswdError => "Username or password error".to_string(),
            Err::EmailExistence => "Email already exists".to_string(),
            Err::InternalError => "Internal server error".to_string(),
            Err::TokenInvalid => "Token is invalid".to_string(),
            Err::TokenExpired => "Token has expired".to_string(),
            Err::RequestNotFound => "Request not found".to_string(),
            Err::DataBaseError => "Database error".to_string(),
            Err::InvalidCsrfToken => "Invalid CSRF token".to_string(),
        }
    }
}

// 实现 IntoResponse trait
impl IntoResponse for Err {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(serde_json::json!({
            "error": self.error_message()
        }));

        (status, body).into_response()
    }
}

// 将 sqlx::Error 转换为 AppError
impl From<sqlx::Error> for Err {
    fn from(_: sqlx::Error) -> Self {
        Err::DataBaseError
    }
}
