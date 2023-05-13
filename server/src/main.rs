use axum::extract::Path;
use hyper::StatusCode;
use serde_json::json;
use simple_db::db::DB;
use axum::{Router, Json, Extension};
use axum::routing::{put, get};
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Mutex;
use std::sync::Arc;

async fn handle_get(key: Path<String>) -> impl IntoResponse {
    println!("accessed");
    println!("{}", key.as_str());
    (
        StatusCode::OK,
        Json(json!({"message":"response!"})),
    )
}

async fn handle_put(Extension(db): Extension<Arc<Mutex<HashMap<String, String>>>>, Path(post_data): Path<String>) -> impl IntoResponse {
    StatusCode::CREATED
}

#[tokio::main]
async fn main() {
    let db = DB::new();

    let app = Router::new()
        .route("/api/:key", get(handle_get))
        .route("/api", put(handle_put)
        .layer(Extension(db.get_arc())));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
