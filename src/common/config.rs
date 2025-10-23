#[derive(Clone, Debug)]

pub struct Server {
    pub host: String,
    pub port: u16,
}

pub struct Config {
    pub database_url: String,
    pub server: Server,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let database_url = std::env::var("DATABASE_URL")?;
        let host = std::env::var("HOST")?;
        let port = std::env::var("PORT")?.parse::<u16>().unwrap_or(8080);

        let server = Server { host, port };

        Ok(Config {
            database_url,
            server,
        })
    }
}
