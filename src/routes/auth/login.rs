use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use bcrypt::verify;
use dotenv::dotenv;
use entity::users::{self};
use jsonwebtoken::{encode, EncodingKey};
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

    let user: users::ActiveModel = users::Entity::find()
        .filter(entity::users::Column::Email.eq(data.email))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .into();

    let password = user.password.unwrap();
    let email = user.email.unwrap();
    let first_name = user.first_name.unwrap();
    let last_name = user.last_name.unwrap();

    let id = user.id.unwrap();

    let valid = verify(data.password, &password).unwrap();

    if valid == true {
        let secret_key =
            std::env::var("JWT_SECRET").expect("JWT_SECRET environment variable not found");
        let key = EncodingKey::from_secret(secret_key.as_bytes());

        let claims = json!({
        "sub": id,
        "email": email,
        });

        let token = encode(&jsonwebtoken::Header::default(), &claims, &key).unwrap();

        return (
            StatusCode::OK,
            Json(json!({
                   "status": 200,
                   "message": "A autenticação foi bem sucedida.",
                   "user": {
                    "id": id,
                    "first_name": first_name,
                    "last_name": last_name,
                    "email": email,
               },
            "token": token
            })),
        );
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "A autenticação falhou" })),
        );
    }
}
