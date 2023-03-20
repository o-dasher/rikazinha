use strum_macros::IntoStaticStr;
use time::OffsetDateTime;

#[derive(IntoStaticStr)]
pub enum TimeFormat {
    #[strum(serialize = "t")]
    ShortTime,
    #[strum(serialize = "T")]
    LongTime,

    #[strum(serialize = "d")]
    ShortDate,
    #[strum(serialize = "D")]
    LongDate,

    #[strum(serialize = "f")]
    ShortDateTime,
    #[strum(serialize = "F")]
    LongDateTime,

    #[strum(serialize = "r")]
    Relative,
}

impl TimeFormat {
    pub fn format(&self, time: &OffsetDateTime) -> String {
        format!(
            "<t:{unix}:{format}>",
            unix = time.unix_timestamp(),
            format = Into::<&str>::into(self)
        )
    }
}
