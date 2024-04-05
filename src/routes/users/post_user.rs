use axum::response::Json;
use bcrypt::hash;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserData {
    first_name: String,
    last_name: String,
    #[validate(email)]
    email: String,
    password: String,
    status: bool,
    role: String,
}

pub async fn post_user(Json(payload): Json<UserData>) -> Json<Value> {
    let mut data: UserData = payload;

    let salt: u32 = 10;
    let hashed_password = hash(data.password, salt).unwrap();

    data.password = hashed_password;
    Json(json!(
    {
        "message": "Usuario criado com sucesso",
        "data": data
    }
    ))
}
