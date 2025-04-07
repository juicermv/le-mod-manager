use dioxus::prelude::*;

use crate::data::ButtonColor;

#[component]
pub fn Button(
    onclick: EventHandler<MouseEvent>,
    color: Option<ButtonColor>,
    children: Element,
) -> Element {
    let finalClr: ButtonColor = match color {
        Some(clr) => clr,
        None => ButtonColor::Primary,
    };

    rsx!(button {
        class: "btn btn-{finalClr}",
        onclick,
        { children }
    })
}
