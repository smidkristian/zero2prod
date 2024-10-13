use secrecy::{ExposeSecret, SecretString};

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn get_connection_string(&self) -> SecretString {
        SecretString::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password.expose_secret(), self.host, self.port, self.database_name
        ).into())
    }

    pub fn get_connection_string_without_db(&self) -> SecretString {
        SecretString::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password.expose_secret(), self.host, self.port
        ).into())
    }
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let config_builder = config::Config::builder();

    // Reads values from project root level file named "config" that `config` lib knows how to parse: yaml, json, etc.
    let config = config_builder
        .add_source(config::File::with_name("config"))
        .build()?;

    config.try_deserialize::<Settings>()
}
