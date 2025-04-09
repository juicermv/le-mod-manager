use dioxus::prelude::*;

use crate::data::{ComponentSizing, Padding, StepUnit};

#[component]
pub fn Container(
    style: Option<String>,
    class: Option<String>,
    padding: Option<Padding>,
    size: Option<ComponentSizing>,
    children: Element,
) -> Element {
    let realSize: ComponentSizing = size.unwrap_or_else(|| ComponentSizing::Fluid);
    let realPadding: Padding =
        padding.unwrap_or_else(|| Padding::all(StepUnit::Positive { value: 2 }));

    let class: String = match realSize {
        ComponentSizing::None => "container".into(),
        _ => "container-".to_string() + &realSize.to_string(),
    } + " "
        + &realPadding.to_classes()
        + " "
        + &class.unwrap_or_default();


    rsx! {
        div {
            class,
            style,
            { children }
        }
    }
}
