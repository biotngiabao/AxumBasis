#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let database_url = std::env::var("DATABASE_URL")?;
        let host = std::env::var("HOST")?;
        let port = std::env::var("PORT")?.parse::<u16>().unwrap_or(8080);
        Ok(Config {
            database_url,
            host,
            port,
        })
    }
}
