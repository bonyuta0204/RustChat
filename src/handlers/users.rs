use crate::entities::prelude::User;
use crate::entities::user::{self};
use crate::errors::ApiError;
use actix_web::{web, HttpResponse, Responder};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

pub async fn get_users(db: web::Data<DatabaseConnection>) -> HttpResponse {
    let users = User::find()
        .all(db.get_ref())
        .await
        .expect("Failed to fetch users");
    HttpResponse::Ok().json(users)
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub provider_id: String,
    pub uid: String,
}

pub async fn create_user(
    db: web::Data<DatabaseConnection>,
    request: web::Json<CreateUserRequest>,
) -> impl Responder {
    let request = request.into_inner();

    // Check if user with the same uid already exists
    let existing_user = User::find()
        .filter(user::Column::Uid.eq(&request.uid))
        .one(db.get_ref())
        .await;

    match existing_user {
        Ok(Some(_)) => Ok(HttpResponse::Conflict().body("User already exists")),
        Ok(None) => {
            let new_user = user::ActiveModel {
                name: Set(request.name),
                email: Set(request.email),
                provider_id: Set(request.provider_id),
                uid: Set(request.uid),
                created_at: Set(chrono::Utc::now().into()),
                updated_at: Set(chrono::Utc::now().into()),
                ..Default::default()
            };

            let res = User::insert(new_user).exec(db.get_ref()).await;
            match res {
                Ok(_) => Ok(HttpResponse::Ok().body("User created")),
                Err(e) => {
                    log::error!("Failed to create user: {:?}", e);
                    Err(ApiError::InternalServerError)
                }
            }
        }
        Err(e) => {
            log::error!("Failed to check existing user: {:?}", e);
            Err(ApiError::InternalServerError)
        }
    }
}
