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
pub struct ApplicationSettings {
    pub host: String,
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

    // layer on the eenvironment-specific values
    let config = config_builder
        .add_source(config::File::from(config_dir.join(environment.as_str())).required(true));

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