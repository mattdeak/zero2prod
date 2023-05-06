use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct PostgresUrl {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Display for PostgresUrl {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}

impl PostgresUrl {
    pub fn from_env() -> Result<PostgresUrl, Box<dyn std::error::Error>> {
        Ok(PostgresUrl {
            host: std::env::var("POSTGRES_HOST")?,
            port: std::env::var("POSTGRES_PORT")?.parse::<u16>()?,
            username: std::env::var("POSTGRES_USER")?,
            password: std::env::var("POSTGRES_PASSWORD")?,
            database: std::env::var("POSTGRES_DB")?,
        })
    }
}
