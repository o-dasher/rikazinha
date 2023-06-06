use anyhow::anyhow;
use chrono::Duration;
use discord_md::{ast::MarkdownDocument, builder::one_line_code};
use poise::{command, serenity_prelude::Colour};

use crate::{
    commands::{
        osu::embeds::{ComposeMode, EmbedField, SimpleMarkDown},
        CommandReturn,
    },
    messages::timestamps::TimeFormat,
    RikaContext,
};

use super::{embeds::Prettify, OsuUsername};

#[command(slash_command)]
pub async fn profile(ctx: RikaContext<'_>, name: OsuUsername) -> CommandReturn {
    let name = name.0;
    let data = ctx.data();

    let player_data = data
        .osu
        .user(&name)
        .await
        .map_err(|_| anyhow!("User `{name}` was not found"))?;

    let statistics = player_data
        .statistics
        .ok_or_else(|| anyhow!("Missing statistics..."))?;

    let accuracy_field =
        EmbedField::new("Accuracy").display(one_line_code(statistics.accuracy.pretty()).md());

    let level_field =
        EmbedField::new("Level").display(one_line_code(statistics.level.float().pretty()).md());

    let playcount_field = EmbedField::new("Playcount")
        .display(one_line_code(statistics.playcount.pretty()).md())
        .info(&format!(
            "{} hrs",
            Duration::seconds(statistics.playtime.into()).num_hours()
        ));

    let medals_field = EmbedField::new("Medals")
        .display(one_line_code(&player_data.medals.unwrap_or_default().len().to_string()).md());

    let peak_rank_field = player_data
        .highest_rank
        .and_then(|highest_rank| {
            EmbedField::new("Peak rank")
                .display(one_line_code(&format!("#{}", highest_rank.rank)).md())
                .info(TimeFormat::ShortDate.format(&highest_rank.updated_at))
                .into()
        })
        .unwrap_or_default();

    let title = format!(
        "{username}: {total_pp:.2}pp (#{global_rank} {country}{country_rank})",
        username = player_data.username,
        total_pp = statistics.pp,
        global_rank = statistics.global_rank.unwrap_or_default(),
        country = player_data.country_code,
        country_rank = statistics.country_rank.unwrap_or_default()
    );

    let description = EmbedField::compose(
        vec![
            EmbedField::compose(vec![accuracy_field, level_field], ComposeMode::Dotted),
            playcount_field,
            medals_field,
            peak_rank_field,
        ],
        ComposeMode::BreakRule,
    );

    ctx.send(|r| {
        r.embed(|e| {
            e.description(description)
                .thumbnail(player_data.avatar_url)
                .author(|a| a.name(title).url(player_data.title_url.unwrap_or_default()))
                .color(Colour::PURPLE)
        })
    })
    .await?;

    Ok(())
}
