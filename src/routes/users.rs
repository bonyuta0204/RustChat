use crate::handlers::users::{create_user, get_users};
use actix_web::{web, HttpResponse, Responder};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(create_user))
            .route(web::get().to(get_users)),
    );
}
