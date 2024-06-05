use config::builder::AsyncState;
use config::{ConfigBuilder, ConfigError, File, FileFormat};
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub discord: DiscordConfig,
    pub fivem: FiveMConfig,
}
#[derive(Deserialize, Clone, Debug)]
pub struct DiscordConfig {
    pub token: String,
    pub prefix: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct FiveMConfig {
    pub server_url: String,
}

impl Config {
    pub async fn load() -> Result<Self, ConfigError> {
        let config = ConfigBuilder::<AsyncState>::default()
            .add_source(File::from(Path::new("config.toml")).format(FileFormat::Toml))
            .build()
            .await?;

        config.try_deserialize().map_err(ConfigError::from)
    }
}