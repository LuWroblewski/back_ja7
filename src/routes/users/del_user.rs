use axum::response::Json;
use serde_json::{json, Value};

pub async fn del_user() -> Json<Value> {
    Json(json!({ "message": "Hello, World! del" }))
}
