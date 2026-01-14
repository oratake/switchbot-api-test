// 以下チュートリアル実施中
// https://programatik29.github.io/axum-tutorial/hello_world.html
use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 環境変数読み込み
    dotenv().expect(".env file not found");

    println!("Fetching devices...");
    get_devices().await?;

    // /hello-worldへのgetリクエストをhello_worldハンドラにパス
    // let app = Router::new().route("/hello-world", get(hello_world));
    let app = Router::new().route("/devices", get(()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn fetch_request() -> anyhow::Result<()> {
    let url = "https://example.com";
    let contents = reqwest::get(url).await?.text().await?;

    println!("text: {:?}", contents);
    Ok(())
}

fn build_auth_headers(token: &str, secret: &str) -> (String, String, String, String) {
    // 1. タイムスタンプ（ミリ秒）を取得
    let t = Utc::now().timestamp_millis().to_string();

    // 2. Nonce（ランダムな文字列）を生成
    let nonce = Uuid::new_v4().to_string();

    // 3. 署名用の文字列を組み立てる (Token + Timestamp + Nonce)
    let data = format!("{}{}{}", token, t, nonce);

    // 4. HMAC-SHA256 で署名を計算
    type HmacSha256 = Hmac<Sha256>;
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());

    // 5. 結果をBase64エンコード
    let result = mac.finalize();
    let signature = general_purpose::STANDARD.encode(result.into_bytes());

    // (Signature, Term, Nonce, Token) を返す
    (signature, t, nonce, token.to_string())
}

//////////
// 構造体
//////////

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct DeviceResponse {
    #[serde(rename = "statusCode")]
    status_code: i32,
    body: DeviceBody,
}

#[derive(Deserialize, Debug)]
struct DeviceBody {
    #[serde(rename = "deviceList")]
    device_list: Vec<Device>,
}

#[derive(Deserialize, Debug)]
struct Device {
    #[serde(rename = "deviceId")]
    device_id: String,
    #[serde(rename = "deviceName")]
    device_name: String,
    #[serde(rename = "deviceType")]
    device_type: String,
}

//////////
// FUNCTIONS
//////////

async fn get_devices() -> anyhow::Result<()> {
    let token_env = env::var("TOKEN").expect("TOKEN not found");
    let secret_env = env::var("SECRET").expect("SECRET not found");

    let (sig, t, nonce, token) = build_auth_headers(&token_env, &secret_env);

    let client = reqwest::Client::new();

    let res = client
        .get("https://api.switch-bot.com/v1.1/devices")
        .header("Authorization", token)
        .header("sign", sig)
        .header("nonce", nonce)
        .header("t", t)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let device_info: DeviceResponse = res.json().await?;

    println!("Status: {}", device_info.status_code);
    for device in device_info.body.device_list {
        println!(
            "Device: [{}] {} (ID: {})",
            device.device_type, device.device_name, device.device_id
        );
    }

    Ok(())
}
