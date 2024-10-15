use backend::model::AuthedUser;
use reqwest::Client;
use reqwest::StatusCode;
#[tokio::main]
async fn main(){
    let res = get_auth(
        "cXFxOjE3Mjg5OTQ5NzA6RXZQbVdOYm1NRlEyRlZwOHp5RGZRalh0M1RpQ0plbDNHcW1EM3AwQ3FMQT0=",
         "http://localhost:8001/"
        );
    match res.await{
        Ok(user) => {
            println!("get user info success: {:?}", user);
        },
        Err(e) => {
            println!("get user info failed: {:?}", e);
        }
    }
}

pub async fn get_auth(
    token: &str,
    url: &str,
) -> anyhow::Result<AuthedUser> {
    let client = Client::new();
    let res = client.get(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/json")
        .send()
        .await.unwrap();
    println!("get user info response: {:?}", res);
    match res.status() {
        StatusCode::OK => {
            let user = res.json::<AuthedUser>().await;
            println!("get user info success: {:?}", user);
            match user{
                Ok(user) => {
                    println!("get user info success:{} ", user.username);
                    Ok(user)
                },
                Err(e) => {
                    println!("get user info failed: {:?}", e);
                    Err(anyhow::Error::msg("get user info failed"))
                }
            }
        },
        StatusCode::UNAUTHORIZED => {
            println!("token invalid");
            Err(anyhow::Error::msg("token invalid"))
        },
        _ => {
            println!("get user info failed");
            Err(anyhow::Error::msg("get user info failed"))
        },
    }
}