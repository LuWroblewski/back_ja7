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

    let first_name: String = user.first_name.unwrap();
    let last_name: String = user.last_name.unwrap();
    let email: String = user.email.unwrap();
    let password: String = user.password.unwrap();
    let status: bool = user.status.unwrap();
    let role: String = user.role.unwrap();

    if status == false {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "O usuario está desativado" })),
        );
    }

    let id: i32 = user.id.unwrap();

    let valid: bool = verify(data.password, &password).unwrap();

    if valid == true {
        let secret_key: String =
            std::env::var("JWT_SECRET").expect("JWT_SECRET environment variable not found");
        let key = EncodingKey::from_secret(secret_key.as_bytes());

        let claims = json!({
        "sub": id,
        "email": email,
        });

        let token: String = encode(&jsonwebtoken::Header::default(), &claims, &key).unwrap();

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
                    "role": role,
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
