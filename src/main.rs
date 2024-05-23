use actix_web::{web, App, HttpServer};
mod db;
// mod handlers;
// mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let db_pool = db::connection::establish_connection()
        .await
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .configure(routes::init_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
