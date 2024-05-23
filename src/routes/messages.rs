use actix_web::{web, HttpResponse, Responder};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/messages")
            .route(web::get().to(get_messages))
            .route(web::post().to(post_message))
    );
}

async fn get_messages() -> impl Responder {
    HttpResponse::Ok().body("Get messages")
}

async fn post_message() -> impl Responder {
    HttpResponse::Ok().body("Post message")
}

