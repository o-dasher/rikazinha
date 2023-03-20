use anyhow::anyhow;
use chrono::Duration;
use poise::{command, serenity_prelude::Colour};

use crate::{
    commands::{
        osu::embeds::{ComposeMode, DisplayTransformer, EmbedField},
        CommandReturn,
    },
    messages::timestamps::TimeFormat,
    RikaContext,
};

use super::{embeds::DisplayValue, OsuUsername};

#[command(slash_command)]
pub async fn profile(ctx: RikaContext<'_>, name: OsuUsername) -> CommandReturn {
    let name = name.value;
    let data = ctx.data();

    let player_data = data
        .osu
        .user(&name)
        .await
        .map_err(|_| anyhow!("User `{name}` was not found"))?;

    let statistics = player_data
        .statistics
        .ok_or_else(|| anyhow!("Missing statistics..."))?;

    let accuracy_and_level_field = EmbedField::compose(
        vec![
            EmbedField::new("Accuracy").display(
                DisplayValue::Decimal(&statistics.accuracy),
                vec![DisplayTransformer::Quoted],
            ),
            EmbedField::new("Level").display(
                DisplayValue::Decimal(&statistics.level.float()),
                vec![DisplayTransformer::Quoted],
            ),
        ],
        ComposeMode::Dotted,
    );

    let playcount_field = EmbedField::new("Playcount")
        .display(
            DisplayValue::Plain(&statistics.playcount.to_string()),
            vec![DisplayTransformer::Quoted],
        )
        .information(
            DisplayValue::Plain(&format!(
                "{} hrs",
                Duration::seconds(statistics.playtime.into()).num_hours()
            )),
            vec![DisplayTransformer::Quoted],
        );

    let medals_field = EmbedField::new("Medals").display(
        DisplayValue::Plain(&player_data.medals.unwrap_or_default().len().to_string()),
        vec![DisplayTransformer::Quoted],
    );

    let peak_rank_field = player_data
        .highest_rank
        .and_then(|highest_rank| {
            EmbedField::new("Peak rank")
                .display(
                    DisplayValue::Plain(&format!("#{}", highest_rank.rank)),
                    vec![DisplayTransformer::Quoted],
                )
                .information(
                    DisplayValue::Plain(&TimeFormat::ShortDate.format(&highest_rank.updated_at)),
                    vec![],
                )
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
            accuracy_and_level_field,
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
