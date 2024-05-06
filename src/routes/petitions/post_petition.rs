use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use dotenv::dotenv;
use entity::petitions::{self};
use sea_orm::Database;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
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

pub async fn post_petition(Json(payload): Json<PetitionData>) -> impl IntoResponse {
    let data: PetitionData = payload;
    insert_petition(&data).await.unwrap();

    return (
        StatusCode::CREATED,
        Json(json!({
          "status": "200",
          "message": "Petição criada com sucesso",
          "petition":{
            "title": data.title,
             "description": data.description,
             "theme": data.theme,
             "type_petition": data.type_petition,
             "status": data.status,
             "file_id": data.file_id,
             "user_last_update": data.user_last_update,
            },
        })),
    );
}

async fn insert_petition(data: &PetitionData) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");

    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();

    let petitions_data: petitions::ActiveModel = petitions::ActiveModel {
        title: Set(data.title.to_owned()),
        description: Set(data.description.to_owned()),
        theme: Set(data.theme.to_owned()),
        type_petition: Set(data.type_petition.to_owned()),
        status: Set(data.status.to_owned()),
        file_id: Set(data.file_id.to_owned()),
        user_last_update: Set(data.user_last_update.to_owned()),
        ..Default::default()
    };

    petitions_data.insert(&db).await?;

    Ok(())
}
