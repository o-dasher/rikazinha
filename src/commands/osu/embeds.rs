use discord_md::{ast::MarkdownDocument, builder::one_line_code};
use std::fmt::Display;

#[derive(Default)]
pub struct EmbedField {
    value: String,
}

pub enum ComposeMode {
    Dotted,
    BreakRule,
}

pub enum DisplayTransformer {
    Quoted,
}

#[derive(Clone, Copy)]
pub enum DisplayValue<'a> {
    Plain(&'a str),
    Decimal(&'a f32),
}

impl<'a> From<&DisplayValue<'a>> for String {
    fn from(value: &DisplayValue<'a>) -> Self {
        match value {
            DisplayValue::Plain(value) => value.to_string(),
            DisplayValue::Decimal(value) => format!("{value:.2}"),
        }
    }
}

impl<'a> DisplayValue<'a> {
    fn display(self, transformers: Vec<DisplayTransformer>) -> String {
        let text = String::from(&self);

        let text_transformers = transformers.iter().map(|transformer| match transformer {
            DisplayTransformer::Quoted => |x: &str| one_line_code(x),
        });

        let transformed_text = text_transformers.fold(text, |acc, f| f(&acc).to_string());

        MarkdownDocument::new(transformed_text).to_string()
    }
}

impl EmbedField {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    pub fn display(
        &self,
        display_value: DisplayValue,
        transformers: Vec<DisplayTransformer>,
    ) -> Self {
        Self::new(&format!(
            "{}: {} ",
            self.value,
            display_value.display(transformers)
        ))
    }

    pub fn information(
        &self,
        display_value: DisplayValue,
        transformers: Vec<DisplayTransformer>,
    ) -> Self {
        Self::new(&format!(
            "{} ({}) ",
            self.value,
            display_value.display(transformers)
        ))
    }

    pub fn compose(fields: Vec<Self>, mode: ComposeMode) -> Self {
        let join_string = match mode {
            ComposeMode::Dotted => " • ",
            ComposeMode::BreakRule => "\n",
        };

        let composed_string = fields
            .iter()
            .map(|f| &*f.value)
            .collect::<Vec<&str>>()
            .join(join_string);

        Self::new(&composed_string)
    }
}

impl Display for EmbedField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
