use secrecy::{ExposeSecret, SecretString};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    // to ensure that value passed through config (string) is correctly parsed as an integer
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.database_name);
        options = options.log_statements(tracing::log::LevelFilter::Trace);
        options
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require 
        } else {
            PgSslMode::Prefer
        };

        // general solution 
        // PgConnectOptions::new()
        //     .username(&self.username)
        //     .password(self.password.expose_secret())
        //     .host(&self.host)
        //     .port(self.port)
        //     .ssl_mode(ssl_mode)

        // google cloud sql
        let url = format!(
            "postgres:///?host={}&port={}&user={}&password={}",
            self.host, self.port, self.username, self.password.expose_secret()
        );

        println!("Database URL: {:?}", url.parse::<PgConnectOptions>());

        url.parse::<PgConnectOptions>().expect("Couldn't parse database URL").ssl_mode(ssl_mode)
    }
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

pub enum Environment {
    Local,
    Production
}

impl Environment {
    pub fn as_str(&self) -> &'static str  {
        match self {
            Environment::Local => "local",
            Environment::Production => "prod"
        }
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let mut config_builder = config::Config::builder();

    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let config_dir = base_path.join("config");

    // read the base config values from project root level file named "config" (that `config` lib knows how to parse: yaml, json, etc.)
    config_builder = config_builder
        .add_source(config::File::from(config_dir.join("base")).required(true));

    let environment: String = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to read APP_ENVIRONMENT environment variable");

    // layer on the environment-specific values
    let config = config_builder
        .add_source(config::File::from(config_dir.join(environment.as_str())).required(true))
        .add_source(config::Environment::with_prefix("APP").separator("__")); // e.g. APP_APPLICATION__PORT will set `Settings.application.port`

    match config.build() {
        Ok(config) => config.try_deserialize::<Settings>(),
        Err(e) => panic!("Failed to build config: {}", e),
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "prod" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment - use either `local` or `prod`",
                other
            )),
        }
    }
}