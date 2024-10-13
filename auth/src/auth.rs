use hmac::{Hmac, Mac};
use tracing::{debug, error, info};
use sha2::Sha256;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_ENGINE;
//BASE64_ENGINE.encode(data) 进行编码
use std::time::{SystemTime, UNIX_EPOCH};
use crate::error::Err;
// 生成 token 函数// 生成 token 函数
pub async fn generate_token(username: &str, secret_key: &str) -> Result<String, Err> {
    let current_time = current_time();
    let token_data = format!("{}:{}", username, current_time);
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()).unwrap();
    mac.update(token_data.as_bytes());
    let signature = mac.finalize().into_bytes();

    // 将 token_data 和 signature 连接成一个字符串
    let token_with_signature = format!("{}:{}", token_data, BASE64_ENGINE.encode(signature.as_slice()));

    let padded_token = BASE64_ENGINE.encode(token_with_signature.as_bytes());

    Ok(padded_token)
}
pub async fn verify_token(token: &str, secret_key: &str,time_out:u64) -> Result<String, Err> {
    //time_out: seconds
    // 确保 token 是有效的 Base64 字符串
    let decoded_token = BASE64_ENGINE.decode(token).map_err(|e| {
        error!("Base64 decode error: {:?}", e);
        Err::TokenInvalid})?;


    let token_str = String::from_utf8(decoded_token).map_err(|e| {
        error!("UTF8 decode error: {:?}", e);
        Err::TokenInvalid})?;
    let token_parts: Vec<&str> = token_str.split(':').collect();
    if token_parts.len() != 3 {
        error!("Token parts length is not 3");
        return Err(Err::TokenInvalid);
    }

    let (username, timestamp_str, signature) = (token_parts[0], token_parts[1], token_parts[2]);

    // 重新生成签名并验证
    let token_data = format!("{}:{}", username, timestamp_str);
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()).unwrap();
    mac.update(token_data.as_bytes());
    let expected_signature = mac.finalize().into_bytes().to_vec();

    // Base64 解码签名
    let decoded_signature = BASE64_ENGINE.decode(signature).map_err(|e| {
        error!("Base64 decode error: {:?}", e);
        Err::TokenInvalid
    })?;

    // 比较签名
    if expected_signature != decoded_signature {
        error!("Signature not match");
        return Err(Err::TokenInvalid);
    }

    // 验证时间戳
    let timestamp = timestamp_str.parse::<u64>().map_err(|_| Err::TokenInvalid)?;

    if current_time() - timestamp > time_out {
        error!("Token expired");
        return Err(Err::TokenExpired);
    }

    Ok(username.to_string())
}
fn current_time() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
