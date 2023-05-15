use axum::extract::Path;
use hyper::StatusCode;
use serde::{Deserialize};
use serde_json::{json, from_str};
use simple_db::db::{DB};
use axum::{Router, Json, Extension};
use axum::routing::{put, get};
use axum::response::IntoResponse;
use std::net::SocketAddr;
use std::println;

#[derive(Deserialize, Debug)]
struct GetInfo{
    key: String
}

async fn handle_get(Extension(db): Extension<DB>, Path(get_data): Path<GetInfo>) -> impl IntoResponse {
    let key = get_data.key;
    println!("{:?}", &key);
    if let Some(v) = db.get(key){
        return (
            StatusCode::OK,
            Json(v)
        )
    } else {
        return (StatusCode::NOT_FOUND, Json("not found".to_string()))
    }
}

#[derive(Deserialize, Debug)]
struct PutInfo {
    key: String,
    value: String,
}

async fn handle_put(Extension(db): Extension<DB>, put_data: String) -> impl IntoResponse {
    println!("{:?}", &put_data);
    let info = put_data;
    // db.set(info.key, info.value);
    StatusCode::CREATED
}

#[tokio::main]
async fn main() {
    let db = DB::new();

    let app = Router::new()
        .route("/api/:key", get(handle_get))
        .layer(Extension(db.clone()))
        .route("/put", put(handle_put)
        .layer(Extension(db.clone())));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
