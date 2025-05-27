use crate::helper::Data;

use database::service::DbService;

mod commands;
mod helper;
use poise::serenity_prelude::{self as serenity};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

// This is the main function that runs the bot
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let mut db = DbService::new();
    // Create the database tables if they don't exist
    db.run_migrations();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                // register(),
                commands::add_truth(),
                commands::add_dare(),
                commands::get_dare(),
                commands::get_truth(),
                commands::accept(),
                commands::reject(),
                commands::delete(),
                commands::list_dares(),
                commands::list_truths(),
                commands::help(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    cooldowns: Arc::new(Mutex::new(HashMap::new())),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
