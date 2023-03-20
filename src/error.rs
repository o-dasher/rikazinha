
use derive_more::From;
use poise::serenity_prelude::{self, Colour};
use thiserror::Error;

#[derive(Debug, From, Error)]
pub enum RikaError {
    #[error(transparent)]
    Serenity(serenity_prelude::Error),

    #[error(transparent)]
    Anyhow(anyhow::Error),
}

pub async fn on_error<U, E: std::fmt::Display + std::fmt::Debug>(
    error: poise::FrameworkError<'_, U, E>,
) -> Result<(), RikaError> {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            tracing::error!("Error while setupping framework: {error}");
        }
        poise::FrameworkError::EventHandler { error, event, .. } => {
            tracing::warn!("EventHandler could not handle event: {error:?} on event {event:?}")
        }
        poise::FrameworkError::Command { error, ctx } => {
            tracing::warn!("FrameworkCommand: {error}");

            ctx.send(|r| r.embed(|e| e.description(error.to_string()).color(Colour::RED)))
                .await?;
        }
        poise::FrameworkError::ArgumentParse { input, .. } => {
            tracing::error!("Parsed unwanted message command on input: {input:?}")
        }
        poise::FrameworkError::CommandStructureMismatch { description, ctx } => {
            tracing::warn!(
                "Failed to deserialize command arguments for command: /{}: {description}",
                ctx.command.name
            )
        }
        poise::FrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
        } => {
            let msg = format!(
                "You are going too fast. Please wait {} seconds...",
                remaining_cooldown.as_secs()
            );

            ctx.send(|b| b.content(msg).ephemeral(true)).await?;
        }
        poise::FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
        } => {
            let msg = format!("I am lacking the following permissions to execute this command: {missing_permissions}");

            ctx.send(|b| b.content(msg).ephemeral(true)).await?;
        }
        poise::FrameworkError::MissingUserPermissions {
            missing_permissions,
            ctx,
        } => {
            let msg = match missing_permissions {
                Some(missing_permissions) => format!("You must have the following permissions to execute this commands: {missing_permissions}"),
                None => format!("You are lacking some permission to execute {}{}", ctx.prefix(), ctx.command().name),
            };

            ctx.send(|b| b.content(msg).ephemeral(true)).await?;
        }
        poise::FrameworkError::NotAnOwner { ctx } => {
            let msg = "Only the bot owners can use this command";

            ctx.send(|b| b.content(msg).ephemeral(true)).await?;
        }
        poise::FrameworkError::GuildOnly { ctx } => {
            let msg = "This command can't be executed on DMs";

            ctx.send(|b| b.content(msg).ephemeral(true)).await?;
        }
        poise::FrameworkError::DmOnly { ctx } => {
            let msg = "This command can only be executed inside DMs";

            ctx.send(|b| b.content(msg).ephemeral(true)).await?;
        }
        poise::FrameworkError::NsfwOnly { ctx } => {
            let msg =
                "This command is really naughty. It can only be executed inside nsfw channels...";

            ctx.send(|b| b.content(msg).ephemeral(true)).await?;
        }
        poise::FrameworkError::CommandCheckFailed { error, .. } => {
            tracing::warn!("Command check failed: {error:?}");
        }
        poise::FrameworkError::DynamicPrefix { error, .. } => {
            tracing::warn!("Dynamic prefix failed: {error:?}")
        }
        poise::FrameworkError::UnknownCommand { msg, .. } => {
            tracing::warn!("Tried to answer an unknown command. msg={msg:?}");
        }
        poise::FrameworkError::UnknownInteraction { interaction, .. } => {
            tracing::warn!("Tried to answer an unkown interaction. interaction={interaction:?}");
        }
        unknown_error => {
            tracing::error!("Not covered error: {unknown_error}");
        }
    }

    Ok(())
}
