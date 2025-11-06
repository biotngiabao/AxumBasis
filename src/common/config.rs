#[derive(Clone, Debug)]

pub struct Server {
    pub host: String,
    pub port: u16,
}

pub struct GprcServer {
    pub host: String,
    pub port: u16,
}

pub struct Config {
    pub database_url: String,
    pub server: Server,
    pub grpc_server: GprcServer,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let database_url = std::env::var("DATABASE_URL")?;
        let host = std::env::var("HOST")?;
        let port = std::env::var("PORT")?.parse::<u16>().unwrap_or(8080);

        let grpc_host = std::env::var("GRPC_HOST")?;
        let grpc_port = std::env::var("GRPC_PORT")?.parse::<u16>().unwrap();

        let server = Server { host, port };
        let grpc_server = GprcServer {
            host: grpc_host,
            port: grpc_port,
        };

        Ok(Config {
            database_url,
            server,
            grpc_server,
        })
    }
}
