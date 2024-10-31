use backend::models::article::*;
use backend::models::catalogue::*;
use backend::models::comment::*;
use backend::models::tag::*;
use backend::models::user::*;
use dotenv::dotenv;
use rand::Rng;
use sqlx::mysql::MySql;
use sqlx::mysql::MySqlPoolOptions;
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

    // 调用 create_user 函数
    // create_user(10, &pool).await.expect("create_user failed");
    // create_tags(&pool).await.expect("create_tags failed");
    create_catalogues(10, &pool)
        .await
        .expect("create_catalogues failed");
    // create_articles(100, &pool).await.expect("create_articles failed");
    // create_resume(10, &pool).await.expect("create_resume failed");
}

async fn create_user(times: u32, pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
    let skills = vec![
        "python",
        "java",
        "c++",
        "rust",
        "go",
        "javascript",
        "typescript",
        "html",
        "css",
        "sql",
    ];
    for i in 1..times + 1 {
        let username = format!("user{}", i);
        let email = format!("{}@example.com", username);
        let nickname = format!("nickname{}", i);
        let avatar = "https://avatars.githubusercontent.com/u/155413462?s=96&v=4";
        let skills = skills.join(",");

        // 插入 user_table 并获取插入的 user_id
        let res = sqlx::query(r#"INSERT INTO user_table (username, email) VALUES (?, ?)"#)
            .bind(&username)
            .bind(&email)
            .execute(pool)
            .await
            .expect("insert user_table failed");

        let user_id = res.last_insert_id();

        // 插入 user_detail_table
        sqlx::query(r#"INSERT INTO user_detail_table (user_id, nickname, avatar, skills) VALUES (?, ?, ?, ?)"#)
            .bind(user_id)
            .bind(&nickname)
            .bind(&avatar)
            .bind(&skills)
            .execute(pool)
            .await
            .expect("insert user_detail_table failed");
    }
    Ok(())
}

async fn create_tags(pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
    let tags = vec![
        "go",
        "python",
        "java",
        "c++",
        "javascript",
        "typescript",
        "html",
        "css",
        "sql",
    ];
    for tag in tags {
        sqlx::query(r#"INSERT INTO tags_table (tag) VALUES (?)"#)
            .bind(tag)
            .execute(pool)
            .await
            .expect("insert tags_table failed");
    }
    Ok(())
}

async fn create_catalogues(times: u32, pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
    for i in 1..times + 1 {
        let catalogue = format!("catalogue{}", i);
        let info = format!("info{}", i);
        // let user_detail_id = rand::thread_rng().gen_range(1..=10);
        let user_detail_id = 6;
        // 插入 catalogues_table
        sqlx::query(
            r#"INSERT INTO catalogues_table (user_detail_id,catalogue, info) VALUES (?, ?,?)"#,
        )
        .bind(user_detail_id)
        .bind(&catalogue)
        .bind(&info)
        .execute(pool)
        .await
        .expect("insert catalogues_table failed");
    }

    Ok(())
}
async fn create_articles(times: u32, pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
    for i in 11..times + 1 {
        let title = format!("title{}", i);
        let content = format!("content{}", i);
        let digest = format!("digest{}", i);
        let user_detail_id = rand::thread_rng().gen_range(1..=10);

        // 插入 articles_table_2024_10
        let res = sqlx::query(r#"INSERT INTO articles_table_2024_10 (title, content,digest, user_detail_id) VALUES (?, ?, ?,?)"#)
            .bind(&title)
            .bind(&content)
            .bind(&digest)
            .bind(user_detail_id)
            .execute(pool)
            .await
            .expect("insert article_table failed");

        let article_id = res.last_insert_id();

        // 插入 article_tags_table
        let tag_id = rand::thread_rng().gen_range(1..=10);
        sqlx::query(r#"INSERT INTO article_tags_table (article_id, tag_id) VALUES (?, ?)"#)
            .bind(article_id)
            .bind(tag_id)
            .execute(pool)
            .await
            .expect("insert article_tag_table failed");

        // 插入 article_catalogues_table
        let catalogue_id = rand::thread_rng().gen_range(1..=10);
        sqlx::query(
            r#"INSERT INTO article_catalogues_table (article_id, catalogue_id) VALUES (?, ?)"#,
        )
        .bind(article_id)
        .bind(catalogue_id)
        .execute(pool)
        .await
        .expect("insert article_catalogues_table failed");
    }

    Ok(())
}

async fn create_resume(times: u32, pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
    for i in 1..times + 1 {
        let content = format!("content{}", i);

        // 插入 resume_table
        sqlx::query(r#"INSERT INTO resume_table (user_detail_id, content) VALUES (?, ?)"#)
            .bind(i as i32)
            .bind(&content)
            .execute(pool)
            .await
            .expect("insert resume_table failed");
    }

    Ok(())
}
