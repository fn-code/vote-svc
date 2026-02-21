use config::{Config, Environment};
use serde::Deserialize;
use std::sync::OnceLock;

static CONFIG: OnceLock<AppConfig> = OnceLock::new();


#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub addr: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub addr: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: String,
    pub max_conn: u32,
    pub min_conn: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppMeta {
    pub env: String,
    pub name: String,
}


#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub app: AppMeta,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl AppConfig {
    // Call once at startup in main.rs
    pub fn init() -> Result<(), config::ConfigError> {
        let cfg = Config::builder()
            .add_source(Environment::default().separator("__"))
            .build()?
            .try_deserialize::<AppConfig>()?;

        CONFIG.set(cfg).ok();
        Ok(())
    }

    // Call from anywhere to get the config
    pub fn global() -> &'static AppConfig {
        CONFIG.get().expect("AppConfig not initialized. Call AppConfig::init() first.")
    }

}

