use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,

    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenvy::dotenv().ok();
        let database_url = env::var("DATABASE_URL")?;
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT")?;

        Ok(Config {
            database_url,
            host,
            port: port.parse().unwrap_or(8080),
        })
    }
}

pub async fn setup_database(
    config: &Config,
) -> Result<sea_orm::DatabaseConnection, sea_orm::DbErr> {
    return sea_orm::Database::connect(&config.database_url).await;
}
