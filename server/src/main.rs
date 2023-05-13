use axum::extract::Path;
use hyper::StatusCode;
use serde_json::json;
use simple_db::db::DB;
use axum::{Router, Json};
use axum::routing::{put, get};
use axum::response::IntoResponse;
use std::net::SocketAddr;

async fn handle_get(key: Path<String>) -> impl IntoResponse {
    println!("accessed");
    println!("{}", key.as_str());
    (
        StatusCode::OK,
        Json(json!({"message":"response!"})),
    )
}

const DB: DB = DB::new(); // クロージャとか使ってmain関数内のdbをキャプチャしたい

async fn handle_put(kv: Path<String>) -> impl IntoResponse {
    StatusCode::CREATED
}

#[tokio::main]
async fn main() {
    // let db = DB::new();

    let app = Router::new()
        .route("/api/:key", get(handle_get))
        .route("/api", put(handle_put));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
