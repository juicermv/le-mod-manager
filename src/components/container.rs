use dioxus::prelude::*;

use crate::data::{ComponentSizing, Padding, StepUnit};

#[component]
pub fn Container(
    padding: Option<Padding>,
    size: Option<ComponentSizing>,
    children: Element,
) -> Element {
    let realSize: ComponentSizing = match size {
        None => ComponentSizing::Fluid,
        Some(s) => s,
    };

    let realPadding: Padding = match padding {
        None => Padding::all(StepUnit::Positive { value: 2 }),
        Some(p) => p,
    };

    let class: String = match realSize {
        _ => "container-".to_string() + &realSize.to_string(),
        ComponentSizing::None => "container".into(),
    } + " "
        + &realPadding.to_classes();

    rsx! {
        div {
            class,
            { children }
        }
    }
}
