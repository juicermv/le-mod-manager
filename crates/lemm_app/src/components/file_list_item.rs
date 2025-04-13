use dioxus::prelude::*;
use lib_lemm::data::PackageMemberType;
use std::path::PathBuf;
use crate::components::{Button, Container};
use crate::data::ButtonColor;

#[component]
pub fn FileListItem(
    file: (PathBuf, PackageMemberType),
    on_delete: EventHandler<PathBuf>,
    on_type_change: EventHandler<(PathBuf, PackageMemberType)>,
) -> Element {
    rsx! {
        div {
            class: "border rounded d-flex flex-row justify-content-between",
            div {
                class: "d-flex flex-row gap-1",

                select {
                    class: "my-auto mx-2",
                    // Options corresponding to the file type
                    option {
                        "Engine Texture"
                    }
                    option {
                        "Texture"
                    }
                    option {
                        "ini"
                    }
                    option {
                        "Config"
                    }
                    option {
                        "cfgpbr"
                    }
                }

                div {
                    class: "vr"
                }

                p {
                    class: "my-auto mx-2",
                    { file.0.to_str().unwrap() }
                }
            }

            Button {
                class: "m-2",
                onclick: move |_| { on_delete(file.0.clone()) },
                color: ButtonColor::Outline(Box::new(ButtonColor::Danger)),
                disabled: false,
                i {
                    class: "bi bi-trash",
                }
            }
        }
    }
}
