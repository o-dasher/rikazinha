use anyhow::{anyhow, bail};
use poise::command;

use crate::{commands::CommandReturn, error::RikaError, RikaContext};

#[command(slash_command)]
pub async fn recent(ctx: RikaContext<'_>) -> CommandReturn {
    let data = ctx.data();

    Ok(())
}
