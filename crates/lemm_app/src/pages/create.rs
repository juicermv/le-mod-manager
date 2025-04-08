use dioxus::prelude::*;

use crate::components::Container;
#[component]
pub fn Create() -> Element {
    rsx! {
        Container {
            h1 { "Create" }
            p { "This is the create page." }
            br {}
            p { "You can create packages here." }
        }
    }
}