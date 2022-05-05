use std::{env, net::SocketAddr};

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Github {
    pub owner: String,
    pub name: String,
    pub branch: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub hostname: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub github: Github,
    pub server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let environment = env::var("WARDEN_ENV").unwrap_or_else(|_| "development".into());

        let settings = Config::builder()
            .add_source(File::with_name("config/warden.json"))
            .add_source(File::with_name(&format!("config/{}", environment)).required(false))
            // Read from local config file; NOTE: DO NOT CHECK INTO GIT
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("warden"))
            .build()?;

        settings.try_deserialize()
    }

    pub fn github(&self) -> &Github {
        &self.github
    }

    pub fn server(&self) -> &Server {
        &self.server
    }

    pub fn server_address(&self) -> SocketAddr {
        format!("{}:{}", &self.server.hostname, &self.server.port)
            .parse()
            .expect("socket address must be valid")
    }
}
