use strum_macros::Display;

#[derive(Display, Clone, PartialEq)]
pub enum ButtonColor {
    #[strum(serialize = "primary")]
    Primary,

    #[strum(serialize = "secondary")]
    Secondary,

    #[strum(serialize = "success")]
    Success,

    #[strum(serialize = "danger")]
    Danger,

    #[strum(serialize = "warning")]
    Warning,

    #[strum(serialize = "info")]
    Info,

    #[strum(serialize = "light")]
    Light,

    #[strum(serialize = "dark")]
    Dark,

    #[strum(serialize = "link")]
    Link,
}
