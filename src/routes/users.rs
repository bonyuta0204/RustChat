use actix_web::{web, HttpResponse, Responder};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(create_user))
    );
}

async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("Create user")
}

