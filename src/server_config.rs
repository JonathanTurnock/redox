use serde::Deserialize;
use config::{Config, Environment, File};

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ServerConfig {
    pub entrypoint: String,
    pub address: String,
    pub port: u16,
    pub keep_alive: u64,
    pub shutdown_timeout: u64,
}

impl ServerConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("server"))
            .add_source(Environment::with_prefix("REDOX"))
            .build()
            .map_err(|e| {
                eprintln!("Failed to build configuration: {}", e);
                e
            })?;

        s.try_deserialize().map_err(|e| {
            eprintln!("Failed to deserialize configuration: {}", e);
            e
        })
    }
}