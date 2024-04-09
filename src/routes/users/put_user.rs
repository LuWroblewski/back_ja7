use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use bcrypt::hash;
use dotenv::dotenv;
use entity::users::{self};
use sea_orm::Database;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserData {
    first_name: String,
    last_name: String,
    #[validate(email(message = "email inválido"))]
    email: String,
    password: String,
    confirm_password: String,
    status: bool,
    role: String,
}

pub async fn put_user(Path(id): Path<u32>, Json(payload): Json<UserData>) -> impl IntoResponse {
    match payload.validate() {
        Ok(_) => {
            let data: UserData = payload;

            if data.password != data.confirm_password {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Senhas não conferem" })),
                );
            }

            let salt: u32 = 10;
            let password: &String = &data.password;
            let hashed_password: String = hash(password, salt).unwrap();
            update_user(&data, hashed_password.clone(), id)
                .await
                .unwrap();

            return (
                StatusCode::OK,
                Json(json!({
                  "status": "200",
                  "message": "Usuário atualizado com sucesso",
                  "user":{
                     "first_name": data.first_name,
                     "last_name": data.last_name,
                     "email": data.email,
                     "password": hashed_password,
                     "status": data.status,
                     "role": data.role,
                    },
                })),
            );
        }
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "erro na validação das informações.", "err": err })),
        ),
    }
}

async fn update_user(
    data: &UserData,
    hashed_password: String,
    id: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

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

    users.first_name = Set(data.first_name.to_owned());
    users.last_name = Set(data.last_name.to_owned());
    users.email = Set(data.email.to_owned());
    users.password = Set(hashed_password.to_owned());
    users.status = Set(data.status.to_owned());
    users.role = Set(data.role.to_owned());

    users.update(&db).await.unwrap();

    Ok(())
}
