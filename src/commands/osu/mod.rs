

use poise::{
    async_trait, command,
    serenity_prelude::{json, CommandOptionType, Context, CreateApplicationCommandOption},
    ApplicationCommandOrAutocompleteInteraction, SlashArgError, SlashArgument,
};

use crate::{commands::CommandReturn, RikaContext};

mod embeds;
mod profile;
mod recent;

use profile::profile;
use recent::recent;

pub struct OsuUsername {
    value: String,
}

#[async_trait]
impl SlashArgument for OsuUsername {
    async fn extract(
        _: &Context,
        _: ApplicationCommandOrAutocompleteInteraction<'_>,
        value: &json::Value,
    ) -> Result<Self, SlashArgError> {
        let string_value = value
            .as_str()
            .ok_or(SlashArgError::CommandStructureMismatch("Expected string"))?;

        Ok(Self {
            value: string_value.into(),
        })
    }

    fn create(builder: &mut CreateApplicationCommandOption) {
        builder
            .name("name")
            .description("The player's username")
            .kind(CommandOptionType::String);
    }
}

#[command(slash_command, subcommands("profile", "recent"))]
pub async fn osu(_ctx: RikaContext<'_>) -> CommandReturn {
    Ok(())
}
