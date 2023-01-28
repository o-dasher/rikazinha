use dotenv::dotenv;
use poise::{
    serenity_prelude::{GatewayIntents, GuildId},
    Framework, FrameworkOptions,
};
use std::env;

pub struct BotPack {}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, BotPack, Error>;

fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Missing environment variable: {}", key))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot_token = get_env("BOT_TOKEN");
    let dev_guild_id_res = env::var("DEV_GUILD_ID");

    let framework = Framework::<BotPack, Error>::builder()
        .options(FrameworkOptions {
            commands: vec![],
            ..Default::default()
        })
        .token(bot_token)
        .intents(GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                let parsed_dev_guild_id = match dev_guild_id_res {
                    Ok(dev_guild_id) => match dev_guild_id.parse::<u64>() {
                        Ok(parsed_dev_guild_id) => Some(GuildId(parsed_dev_guild_id)),
                        Err(_) => None,
                    },
                    Err(_) => None,
                };

                #[derive(Debug)]
                enum RegisterCommandError {
                    Internal,
                    InvalidDevelomentGuildID,
                }

                let register_command_result = match parsed_dev_guild_id {
                    Some(dev_guild_id) => {
                        let commands = &framework.options().commands;
                        let register =
                            poise::builtins::register_in_guild(ctx, commands, dev_guild_id);

                        match register.await {
                            Ok(_) => Ok(()),
                            Err(_) => Err(RegisterCommandError::Internal),
                        }
                    }
                    _ => Err(RegisterCommandError::InvalidDevelomentGuildID),
                };

                if let Err(why) = register_command_result {
                    paris::error!("{:?}", why);
                }

                let bot_pack = BotPack {};

                paris::info!("Finished creating BotPack!");

                Ok(bot_pack)
            })
        });

    paris::info!("Starting up bot...");

    match framework.run().await {
        Ok(_) => {
            paris::info!("The bot is ready!");
        }
        Err(why) => {
            panic!("{}", why)
        }
    }
}
