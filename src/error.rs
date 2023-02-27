use derive_more::From;
use poise::serenity_prelude;
use strum_macros::Display;
use thiserror::Error;

#[derive(Debug, Display, From, Error)]
pub enum RikaError {
    Serenity(serenity_prelude::Error),
    Anyhow(anyhow::Error),
}

pub async fn on_error<U, E: std::fmt::Display + std::fmt::Debug>(
    error: poise::FrameworkError<'_, U, E>,
) -> Result<(), RikaError> {
    match error {
        poise::FrameworkError::Setup {
            error,
            framework,
            data_about_bot,
            ctx,
        } => todo!(),
        poise::FrameworkError::EventHandler {
            error,
            ctx,
            event,
            framework,
        } => todo!(),
        poise::FrameworkError::Command { error, ctx } => todo!(),
        poise::FrameworkError::ArgumentParse { error, input, ctx } => todo!(),
        poise::FrameworkError::CommandStructureMismatch { description, ctx } => todo!(),
        poise::FrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
        } => todo!(),
        poise::FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
        } => todo!(),
        poise::FrameworkError::MissingUserPermissions {
            missing_permissions,
            ctx,
        } => todo!(),
        poise::FrameworkError::NotAnOwner { ctx } => todo!(),
        poise::FrameworkError::GuildOnly { ctx } => todo!(),
        poise::FrameworkError::DmOnly { ctx } => todo!(),
        poise::FrameworkError::NsfwOnly { ctx } => todo!(),
        poise::FrameworkError::CommandCheckFailed { error, ctx } => todo!(),
        poise::FrameworkError::DynamicPrefix { error, ctx, msg } => todo!(),
        poise::FrameworkError::UnknownCommand {
            ctx,
            msg,
            prefix,
            msg_content,
            framework,
            invocation_data,
            trigger,
        } => todo!(),
        poise::FrameworkError::UnknownInteraction {
            ctx,
            framework,
            interaction,
        } => todo!(),
        _ => todo!(),
    }
    Ok(())
}
