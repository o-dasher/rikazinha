use dotenv::dotenv;
use error::RikaError;
use poise::serenity_prelude::GatewayIntents;
use poise::{Framework, FrameworkOptions};
use rosu_v2::prelude::*;
use startup::setup;
use tracing::error;
use tracing_subscriber::fmt::Subscriber;
use utils::env::EnvVar;

mod commands;
mod startup;
mod utils;
mod error;

pub struct RikaData {
    osu: Osu,
}

pub type Context<'a> = poise::Context<'a, RikaData, RikaError>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot_token = EnvVar::BotToken.get().unwrap();

    Subscriber::builder().try_init().unwrap();

    let result = Framework::<RikaData, RikaError>::builder()
        .options(FrameworkOptions {
            commands: vec![],
            on_error: |err| {
                Box::pin(async move {
                    if let Err(e) = error::on_error(err).await {
                        error!("{e:?}");
                    }
                })
            },
            ..Default::default()
        })
        .token(bot_token)
        .intents(GatewayIntents::non_privileged())
        .setup(move |ctx, ready, framework| Box::pin(setup(ctx, ready, framework)))
        .run()
        .await;

    if let Err(e) = result {
        panic!("{}", e)
    }
}
