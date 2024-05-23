use actix_web::{
    web::{Data, JsonConfig},
    App, HttpServer,
};
use errors::ApiError;
mod db;
mod entities;
mod errors;
mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let db_pool = db::connection::establish_connection()
        .await
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let json_config = JsonConfig::default()
            .error_handler(|err, _req| ApiError::BadRequest(err.to_string()).into());

        App::new()
            .app_data(Data::new(db_pool.clone()))
            .app_data(json_config)
            .configure(routes::init_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
