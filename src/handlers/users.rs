use crate::entities::prelude::User;
use actix_web::{web, HttpResponse};
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn get_users(db: web::Data<DatabaseConnection>) -> HttpResponse {
    let users = User::find()
        .all(db.get_ref())
        .await
        .expect("Failed to fetch users");
    HttpResponse::Ok().json(users)
}
