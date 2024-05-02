use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Json};
use dotenv::dotenv;
use entity::users::{self, Model as UserModel};
use sea_orm::Database;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    id: String,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    status: bool,
    role: String,
}

pub async fn get_all_users() -> impl IntoResponse {
    dotenv().ok();

    let database_url: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");

    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();

    let users: Vec<UserModel> = users::Entity::find()
        .all(&db)
        .await
        .unwrap()
        .into_iter()
        .map(|user| UserModel {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            password: user.password,
            status: user.status,
            role: user.role,
            date_create: user.date_create,
            date_last_update: user.date_last_update,
        })
        .collect();

    let serialized_data = json!({
        "users": users.iter().map(|user| {
            json!({
                "id": user.id,
                "first_name": user.first_name,
                "last_name": user.last_name,
                "email": user.email,
                "status": user.status,
                "role": user.role,
                "date_create": user.date_create,
                "date_last_update": user.date_last_update
            })
        }).collect::<Vec<_>>()
    });

    return (
        StatusCode::OK,
        Json(json!({              
    "status": "200",
    "message": "Todos os usuarios carregados com sucesso.", 
    "records": serialized_data })),
    );
}
