use std::sync::Arc;
use clap::Parser;
use env_logger::Env;
use lazy_static::lazy_static;
use log::{error, info};

use serenity::prelude::*;
use nooklicense_bot::commands::command_event_handler::CommandEventHandler;
use nooklicense_bot::config::config::{Data, load};
use nooklicense_bot::ConfigStore;

#[derive(Debug, Parser)]
pub struct CliArgs {
    #[arg(long, default_value = "./config.toml")]
    config: String,
    #[arg(long)]
    debug: bool,
}

lazy_static! {
    pub static ref ARGS: CliArgs = CliArgs::parse();

    pub static ref CONFIG: Data = load(ARGS.config.as_str());
}

#[tokio::main]
async fn main() {
    if ARGS.debug {
        std::env::set_var("RUST_LOG", "debug, serenity=info, tracing::span=off");
    }

    std::env::set_var("RUST_LOG", "serenity=info, tracing::span=off, serenity::http=off,  serenity::gateway::shard=off");


    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Starting bot...");

    let intents = GatewayIntents::all();

    let mut client = Client::builder(&CONFIG.bot.token, intents)
        .event_handler(CommandEventHandler)
        .intents(intents)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("An error occurred while running the client: {:?}", why)
    }

    {
        let mut data = client.data.write().await;

        data.insert::<ConfigStore>(Arc::new(CONFIG.clone()));
    }
}
