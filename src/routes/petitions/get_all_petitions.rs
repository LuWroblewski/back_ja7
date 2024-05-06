use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Json};
use dotenv::dotenv;
use entity::petitions::{self, Model as PetitionModel};
use sea_orm::Database;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::json;


pub async fn get_all_petitions() -> impl IntoResponse {
    dotenv().ok();

    let database_url: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");

    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();

    let petitions: Vec<PetitionModel> = petitions::Entity::find()
        .all(&db)
        .await
        .unwrap()
        .into_iter()
        .map(|petitions| PetitionModel {
            id: petitions.id,
            title: petitions.title,
            description: petitions.description,
            theme: petitions.theme,
            type_petition: petitions.type_petition,
            status: petitions.status,
            file_id: petitions.file_id,
            user_last_update: petitions.user_last_update,
            date_create: petitions.date_create,
            date_last_update: petitions.date_last_update,
        })
        .collect();

    let serialized_data = json!({
        "petitions": petitions.iter().map(|petition| {
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
    "message": "Todas as petições foram carregadas com sucesso.", 
    "records": serialized_data })),
    );
}
