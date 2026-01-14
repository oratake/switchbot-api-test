// 以下チュートリアル実施中
// https://programatik29.github.io/axum-tutorial/hello_world.html
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
