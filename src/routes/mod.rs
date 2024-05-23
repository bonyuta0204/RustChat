use actix_web::web;

mod messages;
mod users;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(messages::init_routes)
            .configure(users::init_routes)
    );
}
