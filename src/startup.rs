use poise::{
    serenity_prelude::{self, GuildId},
    Framework, FrameworkError,
};
use rosu_v2::{prelude::OsuError, Osu};
use tracing::info;

use crate::{
    error::RikaError,
    utils::env::{EnvVar, OsuEnvVar},
    RikaData,
};

async fn create_osu_client() -> Result<Osu, OsuError> {
    let osu_client_id: u64 = EnvVar::Osu(OsuEnvVar::ClientId).get_parsed().unwrap();
    let osu_client_secret = EnvVar::Osu(OsuEnvVar::ClientSecret).get().unwrap();

    Osu::new(osu_client_id, osu_client_secret).await
}

pub fn propagate_error(
    error: RikaError,
    ctx: &serenity_prelude::Context,
    ready: &serenity_prelude::Ready,
    framework: &Framework<RikaData, RikaError>,
) {
    (framework.options().on_error)(FrameworkError::Setup {
        error,
        framework,
        data_about_bot: ready,
        ctx,
    });
}

pub async fn setup(
    ctx: &serenity_prelude::Context,
    ready: &serenity_prelude::Ready,
    framework: &Framework<RikaData, RikaError>,
) -> Result<RikaData, RikaError> {
    let registered = match EnvVar::DevGuild.get_parsed() {
        Err(why) => Err(why),
        Ok(dev_guild_id) => {
            let commands = &framework.options().commands;

            Ok(poise::builtins::register_in_guild(ctx, commands, GuildId(dev_guild_id)).await?)
        }
    };

    match registered {
        Ok(_) => info!("Finished registering commands to development guild"),
        Err(why) => propagate_error(why.into(), ctx, ready, framework),
    }

    let osu_client = create_osu_client().await.unwrap();

    Ok(RikaData { osu: osu_client })
}
