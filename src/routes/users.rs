use crate::handlers::users::get_users;
use actix_web::{web, HttpResponse, Responder};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(create_user))
            .route(web::get().to(get_users)),
    );
}

async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("Create user")
}
