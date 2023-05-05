use std::default::Default;
use std::env;
use std::error::Error;

pub struct DatabaseURL(String);

pub struct DatabaseURLBuilder {
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    password: Option<String>,
    database: Option<String>,
}

impl DatabaseURLBuilder {
    pub fn new() -> DatabaseURLBuilder {
        DatabaseURLBuilder {
            host: None,
            port: None,
            user: None,
            password: None,
            database: None,
        }
    }

    pub fn host(&mut self, host: String) -> &mut Self {
        self.host = Some(host);
        self
    }

    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = Some(port);
        self
    }

    pub fn user(&mut self, user: String) -> &mut Self {
        self.user = Some(user);
        self
    }

    pub fn password(&mut self, password: String) -> &mut Self {
        self.password = Some(password);
        self
    }

    pub fn database(&mut self, database: String) -> &mut Self {
        self.database = Some(database);
        self
    }

    pub fn build(&self) -> DatabaseURL {
        let host = self.host.unwrap_or("localhost".to_owned());
        let port = self.port.unwrap_or(5432);
        let user = self.user.unwrap_or("postgres".to_owned());
        let password = self.password.unwrap_or("".to_owned());
        let db = self.database.unwrap_or("postgres".to_owned());

        DatabaseURL(format!(
            "postgresql://{}:{}@{}:{}/{}",
            user, password, host, port, db
        ))
    }
}

fn get_db_url_from_env() -> Result<DatabaseURL, Box<dyn Error>> {
    let host = env::var("POSTGRES_HOST")?;
    let user = env::var("POSTGRES_USER")?;
    let port = env::var("POSTGRES_PORT")?.parse()?;
    let db = env::var("POSTGRES_DB")?;
    let password = env::var("POSTGRES_PASSWORD")?;

    Ok(DatabaseURLBuilder::new()
        .host(host)
        .port(port)
        .user(user)
        .password(password)
        .database(db)
        .build())
}
