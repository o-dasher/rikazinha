
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
    let registered = EnvVar::DevGuild.get_parsed().map(|dev_guild_id| {
        let commands = &framework.options().commands;

        poise::builtins::register_in_guild(ctx, commands, GuildId(dev_guild_id))
    });

    match registered {
        Ok(future) => {
            future.await?;

            info!("Finished registering commands to development guild");
        }
        Err(why) => propagate_error(why.into(), ctx, ready, framework),
    }

    let osu_client = create_osu_client().await.map_err(anyhow::Error::msg)?;

    Ok(RikaData { osu: osu_client })
}
