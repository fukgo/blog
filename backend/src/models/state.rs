use crate::models::user::User;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};
use std::sync::Mutex;

#[derive(Debug)]
pub struct AppState {
    pub pool: MySqlPool,
    pub user_vec: Mutex<Vec<User>>,
}
