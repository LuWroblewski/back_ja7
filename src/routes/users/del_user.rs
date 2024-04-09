use axum::{extract::Path, http::StatusCode, response::Json};
use dotenv::dotenv;
use entity::users::{self};
use sea_orm::Database;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    status: bool,
}

pub async fn del_user(
    Path(id): Path<u32>,
    Json(payload): Json<UserData>,
) -> (StatusCode, Json<serde_json::Value>) {
    dotenv().ok();

    let data: UserData = payload;

    let database_url: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");

    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();

    let mut users: users::ActiveModel = users::Entity::find()
        .filter(entity::users::Column::Id.eq(id))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .into();

    users.status = Set(data.status);

    users.update(&db).await.unwrap();

    let message: &str = match data.status {
        true => "Usuario ativado",
        false => "Usuario desativado",
    };

    return (StatusCode::OK, Json(json!({ "message": message })));
}
