
use poise::command;

use crate::{commands::CommandReturn, RikaContext};

#[command(slash_command)]
pub async fn recent(ctx: RikaContext<'_>) -> CommandReturn {
    let _data = ctx.data();

    Ok(())
}
