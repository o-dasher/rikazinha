

use anyhow::anyhow;
use poise::{
    builtins::{register_globally, register_in_guild},
    serenity_prelude::{self, GuildId},
    Framework, FrameworkError,
};
use rosu_v2::{prelude::OsuError, Osu};
use tracing::info;

use crate::{error::RikaError, utils::env::EnvVar, RikaData};

async fn create_osu_client() -> Result<Osu, OsuError> {
    let osu_client_id: u64 = EnvVar::OsuClientId.get_parsed().unwrap();
    let osu_client_secret = EnvVar::OsuClientSecret.get().unwrap();

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

enum RegisterType<T> {
    Globally(T),
    OnGuild(T),
}

pub async fn setup(
    ctx: &serenity_prelude::Context,
    ready: &serenity_prelude::Ready,
    framework: &Framework<RikaData, RikaError>,
) -> Result<RikaData, RikaError> {
    let commands = &framework.options().commands;

    let registered = match EnvVar::DevGuild.get_parsed() {
        Ok(dev_guild_id) => {
            RegisterType::OnGuild(register_in_guild(ctx, commands, GuildId(dev_guild_id)).await)
        }
        Err(..) => RegisterType::Globally(register_globally(ctx, commands).await),
    };

    let future = match registered {
        RegisterType::Globally(future) => {
            info!("Finished register commands globally");
            future
        }
        RegisterType::OnGuild(future) => {
            info!("Finished registering commands to development guild");
            future
        }
    };

    if let Err(..) = future {
        propagate_error(
            anyhow!("Failed to register commands...").into(),
            ctx,
            ready,
            framework,
        )
    }

    let osu_client = create_osu_client().await.map_err(anyhow::Error::msg)?;

    Ok(RikaData { osu: osu_client })
}
