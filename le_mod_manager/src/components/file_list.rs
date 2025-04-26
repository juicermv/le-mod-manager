use crate::components::{Button, Container, FileListItem};
use crate::data::ButtonColor;
use dioxus::prelude::*;
use lib_lemm::data::PackageMemberType;
use std::path::PathBuf;
use strum::IntoEnumIterator;

#[component]
pub fn FileList(
    files: Vec<(PathBuf, PackageMemberType)>,
    on_delete: EventHandler<PathBuf>,
    on_type_change: EventHandler<(PathBuf, PackageMemberType)>,
    filter: Option<PackageMemberType>,
    on_filter_change: EventHandler<Option<PackageMemberType>>,
    on_add: EventHandler,
    on_add_engine_texture: EventHandler
) -> Element {
    let filtered_files = files
        .iter()
        .filter(|(_, f_type)| match filter {
            Some(filter_type) => f_type == &filter_type,
            None => true,
        })
        .collect::<Vec<_>>();

    let file_types = PackageMemberType::iter().collect::<Vec<_>>();
    rsx! {
        div { class: "d-flex flex-column rounded border p-0",
            div { class: "bg-secondary-subtle text-secondary-emphasis rounded-top d-flex flex-row border-bottom p-2 d-flex justify-content-between gap-2 flex-wrap",
                div {
                    class: "btn-group my-auto",
                    Button {onclick: move |_| on_add(()), "Add Files..." i { class: "bi bi-box-arrow-up-right ms-2" } }
                    Button {onclick: move |_| on_add_engine_texture(()), "Add Engine Textures..." i { class: "bi bi-box-arrow-up-right ms-2" } }
                }

                div { class: "d-flex flex-row gap-2",
                    p { class: "my-auto", "Filter: " }
                    div { class: "btn-group",
                        Button {
                            class: if filter.is_none() { Some("active".to_string()) } else { None },
                            color: ButtonColor::Outline(Box::new(ButtonColor::Secondary)),
                            onclick: move |_| on_filter_change(None),
                            "None"
                        }
                        for file_type in file_types {
                            Button {
                                class: if filter == Some(file_type) { Some("active".to_string()) } else { None },
                                color: ButtonColor::Outline(Box::new(ButtonColor::Secondary)),
                                onclick: move |_| on_filter_change(Some(file_type)),
                                {<PackageMemberType as Into<String>>::into(file_type)}
                            }
                        }
                    }
                }
            }

            div { class: "d-flex flex-column  p-2 gap-2 overflow-auto",
                style: "max-height: 50vh;",
                if !filtered_files.is_empty() {
                    for file in filtered_files {
                        FileListItem {
                            file: file.clone(),
                            on_delete,
                            on_type_change,
                        }
                    }
                } else {
                    div {
                        class: "py-4 mx-auto",
                        h1 {
                            class: "text-secondary",
                            "Nothing here yet..."
                        }
                    }
                }
            }
        }
    }
}
