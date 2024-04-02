use axum::response::Json;
use serde_json::{json, Value};

pub async fn get_user() -> Json<Value> {
    Json(json!({ "message": "Hello, World! Get" }))
}
