use strum_macros::Display;

#[derive(Display, Clone, PartialEq)]
pub enum ComponentSizing {
    None,

    #[strum(serialize = "sm")]
    Small,

    #[strum(serialize = "md")]
    Medium,

    #[strum(serialize = "lg")]
    Large,

    #[strum(serialize = "xl")]
    ExtraLarge,

    #[strum(serialize = "xxl")]
    ExtraExtraLarge,

    #[strum(serialize = "fluid")]
    Fluid,
}

#[derive(Display, Clone, PartialEq)]
pub enum StepUnit{
    #[strum(serialize = "n{0}")]
    Negative(u8),

    #[strum(serialize = "{0}")]
    Positive(u8),

    #[strum(serialize = "auto")]
    Auto,
}

impl Default for StepUnit {
    fn default() -> Self {
        StepUnit::Positive(0)
    }
}
