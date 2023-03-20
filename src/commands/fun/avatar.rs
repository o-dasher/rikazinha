use poise::command;

use crate::{commands::CommandReturn, RikaContext};

#[command(slash_command)]
pub async fn avatar(ctx: RikaContext<'_>) -> CommandReturn {
    Ok(())
}
