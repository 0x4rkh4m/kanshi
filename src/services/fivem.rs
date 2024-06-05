use reqwest::{Client};
use serde_json::Value;
use crate::config::Config;
use poise::futures_util::lock::Mutex as PoiseMutex;
use std::sync::Arc;
use std::fmt;

#[derive(Debug)]
pub enum FiveMServiceError {
    UrlNotConfigured,
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for FiveMServiceError {
    fn from(err: reqwest::Error) -> FiveMServiceError {
        FiveMServiceError::ReqwestError(err)
    }
}

pub struct FiveMService {
    client: Client,
    config: Arc<PoiseMutex<Config>>,
}

impl fmt::Debug for FiveMService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FiveMService")
            .field("config", &self.config)
            .finish()
    }
}

impl FiveMService {
    pub fn new(config: Arc<PoiseMutex<Config>>) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn set_server_url(&self, url: &str) {
        let mut config = self.config.lock().await;
        config.fivem.server_url = url.to_string();
    }

    pub async fn get_server_url(&self) -> String {
        let config = self.config.lock().await;
        config.fivem.server_url.clone()
    }

    pub async fn get_server_info(&self) -> Result<Value, FiveMServiceError> {
        let config = self.config.lock().await;
        if config.fivem.server_url.is_empty() {
            return Err(FiveMServiceError::UrlNotConfigured);
        }

        let server_info: Value = self.client.get(&format!("{}/info.json", config.fivem.server_url))
            .send()
            .await?
            .json()
            .await?;
        Ok(server_info)
    }

    pub async fn get_player_info(&self) -> Result<Value, FiveMServiceError> {
        let config = self.config.lock().await;
        if config.fivem.server_url.is_empty() {
            return Err(FiveMServiceError::UrlNotConfigured);
        }

        let player_info: Value = self.client.get(&format!("{}/players.json", config.fivem.server_url))
            .send()
            .await?
            .json()
            .await?;
        Ok(player_info)
    }
}