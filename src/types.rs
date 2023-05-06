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
