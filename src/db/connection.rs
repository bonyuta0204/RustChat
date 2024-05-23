use sea_orm::DatabaseConnection;
use sea_orm::Database;

pub async fn establish_connection() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(&database_url).await
}
