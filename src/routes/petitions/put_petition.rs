use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use dotenv::dotenv;
use entity::petitions::{self};
use sea_orm::Database;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct PetitionData {
    description: String,
    title: String,
    theme: String,
    type_petition: String,
    status: String,
    file_id: i32,
    user_last_update: i32,
}

pub async fn put_petition(
    Path(id): Path<u32>,
    Json(payload): Json<PetitionData>,
) -> impl IntoResponse {
    let data: PetitionData = payload;

    update_user(&data, id).await.unwrap();

    return (
        StatusCode::OK,
        Json(json!({
          "status": "200",
          "message": "Petição atualizada com sucesso",
          "petition":{
            "title": data.title,
             "description": data.description,
             "theme": data.theme,
             "type_petition": data.type_petition,
             "status": data.status,
             "file_id": data.file_id,
             "user_last_update": data.user_last_update
            },
        })),
    );
}

async fn update_user(data: &PetitionData, id: u32) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");

    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();

    let mut petitions: petitions::ActiveModel = petitions::Entity::find()
        .filter(entity::users::Column::Id.eq(id))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .into();

    petitions.title = Set(data.title.to_owned());
    petitions.description = Set(data.description.to_owned());
    petitions.theme = Set(data.theme.to_owned());
    petitions.type_petition = Set(data.type_petition.to_owned());
    petitions.status = Set(data.status.to_owned());
    petitions.file_id = Set(data.file_id.to_owned());
    petitions.user_last_update = Set(data.user_last_update.to_owned());

    petitions.update(&db).await.unwrap();

    Ok(())
}
