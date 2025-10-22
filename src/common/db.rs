use sea_orm::{ Database, DatabaseConnection };

use crate::common::config;

pub async fn setup_database(config: &config::Config) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let db: Result<DatabaseConnection, sea_orm::DbErr> = Database::connect(
        &config.database_url
    ).await;

    println!("Connected to database successfully at {}", &config.database_url);
    return db;
}
