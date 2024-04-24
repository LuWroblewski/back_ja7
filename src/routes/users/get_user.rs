use axum::{extract::Path, http::StatusCode, response::Json};
use dotenv::dotenv;
use entity::users::{self, Model as UserModel};
use sea_orm::Database;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::json;

pub async fn get_user(Path(id): Path<u32>) -> (StatusCode, Json<serde_json::Value>) {
    dotenv().ok();

    let database_url: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");

    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();

    let users: Vec<UserModel> = users::Entity::find()
        .filter(entity::users::Column::Id.eq(id))
        .one(&db)
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
                "password": user.password,
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
    "message": "Usuario carregado com sucesso.", 
    "record": serialized_data })),
    );
}
