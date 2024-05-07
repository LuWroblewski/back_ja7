use axum::{extract::Path, http::StatusCode, response::Json};
use dotenv::dotenv;
use entity::petitions::{self};
use sea_orm::Database;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct PetitionData {
    status: String,
}

pub async fn del_petition(
    Path(id): Path<u32>,
    Json(payload): Json<PetitionData>,
) -> (StatusCode, Json<serde_json::Value>) {
    dotenv().ok();

    let data: PetitionData = payload;

    let database_url: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");

    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();

    let mut petitions: petitions::ActiveModel = petitions::Entity::find()
        .filter(entity::petitions::Column::Id.eq(id))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .into();

    petitions.status = Set(data.status);

    petitions.update(&db).await.unwrap();

    return (
        StatusCode::OK,
        Json(json!({
            "status": "200",
            "message": "Petição excluida com sucesso",
        })),
    );
}
