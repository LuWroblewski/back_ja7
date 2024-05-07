use axum::{extract::Path, http::StatusCode, response::Json};
use dotenv::dotenv;
use entity::petitions::{self, Model as PetitionModel};
use sea_orm::Database;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::json;

pub async fn get_petition(Path(id): Path<u32>) -> (StatusCode, Json<serde_json::Value>) {
    dotenv().ok();

    let database_url: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");

    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();

    let petitions: Vec<PetitionModel> = petitions::Entity::find()
        .filter(entity::petitions::Column::Id.eq(id))
        .one(&db)
        .await
        .unwrap()
        .into_iter()
        .map(|petition| PetitionModel {
            id: petition.id,
            title: petition.title,
            description: petition.description,
            theme: petition.theme,
            type_petition: petition.type_petition,
            status: petition.status,
            file_id: petition.file_id,
            user_last_update: petition.user_last_update,
            date_create: petition.date_create,
            date_last_update: petition.date_last_update,
        })
        .collect();

    let serialized_data = json!({
        "petition": petitions.iter().map(|petition| {
            json!({
                "id": petition.id,
                "title": petition.title,
                "description": petition.description,
                "theme": petition.theme,
                "type_petition": petition.type_petition,
                "status": petition.status,
                "file_id": petition.file_id,
                "user_last_update": petition.user_last_update,
                "date_create": petition.date_create,
                "date_last_update": petition.date_last_update
            })
        }).collect::<Vec<_>>()
    });

    return (
        StatusCode::OK,
        Json(json!({
            "status": "200",
            "message": "Petição carregada com sucesso.",
            "record": serialized_data
        })),
    );
}
