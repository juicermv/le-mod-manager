use crate::{components::Container, data::AppState};
use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    let filepath = use_context::<AppState>().filepath;
    rsx! {
        Container {
            p {
                "Current filepath: {filepath}"
            }
        }
    }
}
