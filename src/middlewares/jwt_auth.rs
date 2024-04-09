use axum::response::Json;
use serde_json::{json, Value};

pub async fn jwt_auth() -> Json<Value> {
    Json(json!({ "message": "Hello, World! auth" }))
}
