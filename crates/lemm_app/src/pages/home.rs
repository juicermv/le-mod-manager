use dioxus::prelude::*;
use rfd::AsyncFileDialog;

use crate::{
    components::{Button, Container},
    data::AppState,
};

#[component]
pub fn Home() -> Element {
    rsx! {
        Container {
            h1 {
                "Dark Souls Lighting Engine Mod Manager"
            }
        }
    }
}
