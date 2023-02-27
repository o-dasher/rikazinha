use std::{
    env,
    fmt::{Debug, Display},
    str::FromStr,
};

use anyhow::Context;
use derive_more::From;
use strum_macros::IntoStaticStr;

#[derive(Clone, Copy)]
pub enum OsuEnvVar {
    ClientId,
    ClientSecret,
}

#[derive(IntoStaticStr, Clone, Copy, From)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum EnvVar {
    BotToken,
    DevGuild,
    Osu(OsuEnvVar),
}

impl EnvVar {
    fn env_var_display(key: &str) -> String {
        format!("environment variable: {key}")
    }

    pub fn get(self) -> Result<String, anyhow::Error> {
        let key: &str = self.into();
        let env_var_display = Self::env_var_display(key);

        env::var(key).context(format!("Missing {env_var_display}"))
    }

    pub fn get_parsed<T: FromStr>(&self) -> Result<T, anyhow::Error>
    where
        T::Err: 'static + Send + Sync + Display + Debug,
    {
        let env_var_display = Self::env_var_display(self.into());

        self.get()?
            .parse()
            .map_err(anyhow::Error::msg)
            .context(format!("Failed to parse {env_var_display}"))
    }
}
