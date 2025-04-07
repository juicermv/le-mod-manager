use dioxus::prelude::*;
use rfd::AsyncFileDialog;

use crate::{
    components::{Button, Container},
    data::AppState,
};

#[component]
pub fn Home() -> Element {
    let mut filepath = use_context::<AppState>().filepath;

    rsx! {
        Container {
            p {
                "File path: {filepath}"
            }
            br {}
            Button {
                onclick: move |_| async move {
                    println!("Button clicked!");
                    let result = AsyncFileDialog::new().set_title("Pick a file...").pick_file().await;

                    let path: String = match result {
                        None => { "".into() }
                        Some(handle) => {
                            handle.path().to_str().unwrap_or_default().to_string()
                        }
                    };

                    filepath.set(path);
                },
                "Click me!"
            }
        }
    }
}
