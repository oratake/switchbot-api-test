// 以下チュートリアル実施中
// https://programatik29.github.io/axum-tutorial/hello_world.html
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use dotenvy::dotenv;
use std::env;

use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // reqwestのテスト
    fetch_request().await?;

    // 環境変数読み込み
    dotenv().expect(".env file not found");
    // /hello-worldへのgetリクエストをhello_worldハンドラにパス
    let app = Router::new().route("/hello-world",get(hello_world));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn hello_world() -> String {
    let testEnv = env::var("TEST-ENV").expect("TEST-ENV not found");

    return format!("test: {testEnv}");
    //"Hello, World!"
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
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(data.as_bytes());
    
    // 5. 結果をBase64エンコード
    let result = mac.finalize();
    let signature = general_purpose::STANDARD.encode(result.into_bytes());

    // (Signature, Term, Nonce, Token) を返す
    (signature, t, nonce, token.to_string())
}
