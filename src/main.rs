mod config;
mod utils;
mod services;
mod commands;

use std::sync::Arc;
use config::Config;
use log::info;
use poise::{serenity_prelude as serenity, Framework, FrameworkError, FrameworkOptions};
use poise::futures_util::lock::Mutex as PoiseMutex;
use serenity::client::ClientBuilder;
use serenity::prelude::GatewayIntents;
use crate::commands::setup_server::{set_server_url, show_server_url};
use crate::services::fivem::FiveMService;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug)]
pub struct Data {
    pub fivem_service: Arc<PoiseMutex<FiveMService>>,
}

#[tokio::main]
async fn main() {
    utils::logger::init();

    let config = Config::load().await.expect("Failed to load configuration");
    let config_arc = Arc::new(PoiseMutex::new(config.clone()));
    let fivem_service = Arc::new(PoiseMutex::new(FiveMService::new(config_arc.clone())));

    let framework = Framework::builder()
        .options(FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(config.discord.prefix.unwrap()),
                ..Default::default()
            },
            on_error: |error: FrameworkError<'_, Data, Error>| {
                Box::pin(async move {
                    eprintln!("Error: {:?}", error);
                })
            },
            commands: vec![
                set_server_url(),
                show_server_url(),
            ],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    fivem_service: fivem_service.clone(),
                })
            })
        })
        .build();

    let token = config.discord.token;
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Failed to create client");

    info!("Kanshi is starting...");

    client.start().await.expect("Failed to start client");
}