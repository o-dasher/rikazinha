use dotenv::dotenv;
use error::RikaError;
use poise::serenity_prelude::GatewayIntents;
use poise::{Framework, FrameworkOptions};
use rosu_v2::prelude::*;
use serde::Deserialize;
use startup::setup;
use tracing::error;
use tracing_subscriber::fmt::Subscriber;

mod commands;
mod error;
mod messages;
mod startup;
mod utils;

#[derive(Deserialize, Debug)]
pub struct Environment {
    bot_token: String,
    dev_guild: Option<u64>,
    osu_client_id: u64,
    osu_client_secret: String,
}

pub struct RikaData {
    osu: Osu,
}

pub type RikaContext<'a> = poise::Context<'a, RikaData, RikaError>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    Subscriber::builder().try_init().unwrap();

    let config = envy::from_env::<Environment>().unwrap();

    let result = Framework::<RikaData, RikaError>::builder()
        .options(FrameworkOptions {
            commands: vec![commands::osu::osu(), commands::fun::avatar::avatar()],
            on_error: |err| {
                Box::pin(async move {
                    if let Err(e) = error::on_error(err).await {
                        error!("{e:?}");
                    }
                })
            },
            ..Default::default()
        })
        .token(&config.bot_token)
        .intents(GatewayIntents::non_privileged())
        .setup(move |ctx, ready, framework| Box::pin(setup(ctx, ready, framework, config)))
        .run()
        .await;

    if let Err(e) = result {
        panic!("{}", e)
    }
}
