use actix_web::{App, HttpServer};

// アプリケーションアーキテクチャー
mod routes;
mod handlers;
mod api;
mod externals;

mod models;
mod config;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HTTPサーバーを起動
    HttpServer::new(move || {
        App::new()
    })
    .bind("0.0.0.0:8080")? // ポート8080でバインド
    .run()
    .await
}
