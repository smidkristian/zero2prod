use dotenv::dotenv;
use std::env;

use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseSettings {
    pub uri: String,
    pub database_name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "prod",
        }
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    dotenv().ok();

    let mut config_builder = config::Config::builder();

    let base_path = env::current_dir().expect("Failed to determine the current directory");
    let config_dir = base_path.join("config");

    // read the base config values from project root level file named "config" (that `config` lib knows how to parse: yaml, json, etc.)
    config_builder =
        config_builder.add_source(config::File::from(config_dir.join("base")).required(true));

    let environment: String = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "local".into());

    // layer on the environment-specific values
    let config = config_builder
        .add_source(config::File::from(config_dir.join(environment.as_str())).required(true))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        ); // e.g. APP_APPLICATION__PORT will set `Settings.application.port`

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
