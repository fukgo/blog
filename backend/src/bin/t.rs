use backend::error::AppError;
use backend::models::user::UserSession;
use dotenv::dotenv;
use rand::Rng;
use sqlx::mysql::MySql;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use sqlx::Pool;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("connect to mysql failed");
    let user = get_session_user_by_username_db(&pool, "qqq").await;
    if let Ok(user) = user {
        println!("user: {:?}", user);
    } else {
        println!("user not found");
    }
}

pub async fn get_session_user_by_username_db(
    pool: &MySqlPool,
    username: &str,
) -> Result<UserSession, AppError> {
    let user = sqlx::query_as::<_, UserSession>(
        r#"
        SELECT 
            b.id AS user_detail_id, 
            u.username AS username, 
            u.email AS email, 
            b.nickname AS nickname, 
            b.avatar AS avatar
        FROM 
            user_table u
        JOIN 
            user_detail_table b 
        ON 
            u.id = b.user_id
        WHERE 
            u.username = ?;
        "#,
    )
    .bind(username)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        println!("get user info failed: {:?}", e);
        AppError::UserNotFound
    })?;
    if let Some(user) = user {
        println!("get user info success");
        Ok(user)
    } else {
        println!("get user info failed");
        Err(AppError::UserNotFound)
    }
}
