
use axum::{
    response::{IntoResponse, Response},
    Json,
    http::StatusCode,
};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
#[derive(Error, Debug,Serialize)]
pub enum AppError {
    #[error("User not found")]
    UserNotFound,
    
    #[error("ChatRoom not found")]
    ChatRoomNotFound,

    #[error("Internal server error")]
    InternalError,
    #[error("User login error")]
    UserLoginError,
    #[error("User is not login")]
    UserUnLogin,
    #[error("User already register")]
    UserAlreadyExist,
    #[error("User already in room")]
    UserAlreadyInRoom,
    #[error("DataBase error")]
    DataBaseError,

    #[error("Token invalid")]
    TokenInvalid,
    #[error("Token expired")]
    TokenExpired,
    #[error("Request not found")]
    RequestNotFound,
    #[error("Nickname already exist")]
    NicknameExist,
    
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            AppError::ChatRoomNotFound => (StatusCode::NOT_FOUND, "ChatRoom not found"),
            AppError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            AppError::UserLoginError => (StatusCode::INTERNAL_SERVER_ERROR, "User login error, please check your username and password"),
            AppError::UserUnLogin => (StatusCode::UNAUTHORIZED, "User is not login"),
            AppError::UserAlreadyExist => (StatusCode::INTERNAL_SERVER_ERROR, "User already register"),
            AppError::UserAlreadyInRoom => (StatusCode::INTERNAL_SERVER_ERROR, "User already in room"),
            AppError::DataBaseError => (StatusCode::INTERNAL_SERVER_ERROR, "DataBase error"),
            AppError::TokenInvalid => (StatusCode::FORBIDDEN, "Token invalid"),
            AppError::TokenExpired => (StatusCode::FORBIDDEN, "Token expired"),
            AppError::RequestNotFound => (StatusCode::NOT_FOUND, "Request not found"),
            AppError::NicknameExist => (StatusCode::INTERNAL_SERVER_ERROR, "Nickname already exist"),
        };

        let body = Json(json!({ "error": error_message }));

        (status, Json(json!({ "error": error_message }))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(_: sqlx::Error) -> Self {
        AppError::InternalError
    }
}


