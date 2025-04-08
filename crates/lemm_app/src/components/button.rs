use dioxus::prelude::*;

use crate::data::ButtonColor;

#[component]
pub fn Button(
    class: Option<String>,
    onclick: EventHandler<MouseEvent>,
    color: Option<ButtonColor>,
    children: Element,
    disabled: Option<bool>,
) -> Element {
    let finalClr: ButtonColor = color.unwrap_or_else(|| ButtonColor::Primary);
    let class = "btn btn-".to_string() + &finalClr.to_string() + " " + &class.unwrap_or_default();

    rsx!(button {
        class,
        disabled,
        onclick,
        { children }
    })
}
