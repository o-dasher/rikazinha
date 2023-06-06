use std::fmt::Display;

use discord_md::ast::{MarkdownDocument, MarkdownElement};
use num_traits::Num;

#[derive(Default)]
pub struct EmbedField(String);

pub enum ComposeMode {
    Dotted,
    BreakRule,
}

pub trait Prettify {
    fn pretty(self) -> String;
}

pub trait SimpleMarkDown {
    fn md(self) -> MarkdownDocument;
}

impl SimpleMarkDown for MarkdownElement {
    fn md(self) -> MarkdownDocument {
        MarkdownDocument::new(self)
    }
}

impl<T: Num + Display> Prettify for T {
    fn pretty(self) -> String {
        format!("{self:.2}")
    }
}

impl EmbedField {
    pub fn new(value: impl ToString) -> Self {
        Self(value.to_string())
    }

    pub fn display(self, value: impl ToString) -> Self {
        Self::new(&format!("{}: {} ", self.0, value.to_string()))
    }

    pub fn info(self, value: impl ToString) -> Self {
        Self::new(&format!("{} ({}) ", self.0, value.to_string()))
    }

    pub fn compose(fields: Vec<Self>, mode: ComposeMode) -> Self {
        let join_string = match mode {
            ComposeMode::Dotted => " • ",
            ComposeMode::BreakRule => "\n",
        };

        let composed_string = fields
            .iter()
            .map(|f| &*f.0)
            .collect::<Vec<&str>>()
            .join(join_string);

        Self::new(composed_string)
    }
}

impl Display for EmbedField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
