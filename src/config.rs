use config::{Config, ConfigError, File};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DbSettings,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct DbSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub db_name: String,
}

impl DbSettings {
    #[must_use]
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.db_name
        ))
    }

    #[must_use]
    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn get_config() -> Result<Settings, ConfigError> {
    Config::builder()
        .add_source(File::with_name("config"))
        .build()?
        .try_deserialize()
}
