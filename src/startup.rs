use anyhow::anyhow;
use poise::{
    builtins::{register_globally, register_in_guild},
    serenity_prelude::{self, GuildId},
    Framework, FrameworkError,
};
use rosu_v2::Osu;
use strum_macros::IntoStaticStr;
use tracing::info;

use crate::{error::RikaError, Environment, RikaData};

pub fn propagate_error(
    error: RikaError,
    ctx: &serenity_prelude::Context,
    ready: &serenity_prelude::Ready,
    framework: &Framework<RikaData, RikaError>,
) {
    let error_handler = framework.options().on_error;

    error_handler(FrameworkError::Setup {
        error,
        framework,
        data_about_bot: ready,
        ctx,
    });
}

#[derive(IntoStaticStr)]
enum RegisterType {
    Globally,
    OnGuild,
}

pub async fn setup(
    ctx: &serenity_prelude::Context,
    ready: &serenity_prelude::Ready,
    framework: &Framework<RikaData, RikaError>,
    config: Environment,
) -> Result<RikaData, RikaError> {
    let commands = &framework.options().commands;

    let (response, register_type) = match config.dev_guild {
        Some(dev_guild) => (
            register_in_guild(ctx, commands, GuildId(dev_guild)).await,
            RegisterType::OnGuild,
        ),
        None => (
            register_globally(ctx, commands).await,
            RegisterType::Globally,
        ),
    };

    info!(
        "Finished registering commands: {}",
        Into::<&'static str>::into(&register_type)
    );

    if let Err(..) = response {
        propagate_error(
            anyhow!("Failed to register commands...").into(),
            ctx,
            ready,
            framework,
        )
    }

    let osu_client = Osu::new(config.osu_client_id, &config.osu_client_secret)
        .await
        .map_err(anyhow::Error::msg)?;

    Ok(RikaData { osu: osu_client })
}
