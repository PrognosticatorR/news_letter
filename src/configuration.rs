//! src/configuration.rs
#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

use config::{Config, File, FileFormat};

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = Config::builder();
    settings = settings.add_source(File::new("configuration", FileFormat::Json));
    let config = settings.build()?;
    let settings: Settings = config.try_deserialize()?;
    Ok(settings)
}
