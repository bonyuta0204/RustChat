use crate::entities::prelude::User;
use crate::entities::user::{self};
use actix_web::{web, HttpResponse, Responder};
use sea_orm::{DatabaseConnection, EntityTrait, Set};
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
}

pub async fn create_user(
    db: web::Data<DatabaseConnection>,
    request: web::Json<CreateUserRequest>,
) -> impl Responder {
    let request = request.into_inner();
    let new_user = user::ActiveModel {
        name: Set(request.name),
        email: Set(request.email),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    let res = User::insert(new_user).exec(db.get_ref()).await;
    match res {
        Ok(_) => HttpResponse::Ok().body("User created"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
    }
}
