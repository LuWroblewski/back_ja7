use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use dotenv::dotenv;
use entity::users::{self};
use jsonwebtoken::{encode, Algorithm, EncodingKey};
use sea_orm::Database;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuth {
    email: String,
    password: String,
}

pub async fn login(Json(payload): Json<UserAuth>) -> impl IntoResponse {
    dotenv().ok();

    let data: UserAuth = payload;

    let database_url: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");

    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();

    let users: users::ActiveModel = users::Entity::find()
        .filter(entity::users::Column::Email.eq(data.email))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .into();

    println!("{:?}", users);

    return (
        StatusCode::OK,
        Json(json!({ "message": "Hello, World! Login" })),
    );
}
