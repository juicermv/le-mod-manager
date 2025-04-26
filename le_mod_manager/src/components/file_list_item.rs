use crate::components::{Button, Container};
use crate::data::ButtonColor;
use dioxus::prelude::*;
use lib_lemm::data::PackageMemberType;
use std::path::PathBuf;
use std::sync::Arc;
use strum::IntoEnumIterator;

#[component]
pub fn FileListItem(
    file: (PathBuf, PackageMemberType),
    on_delete: EventHandler<PathBuf>,
    on_type_change: EventHandler<(PathBuf, PackageMemberType)>,
) -> Element {
    let p_buff = Arc::new(file.0.clone());
    let p_buff_2 = Arc::clone(&p_buff);

    let directory = file.0.parent().unwrap().to_str().unwrap_or_default().to_string() + if std::env::consts::OS == "windows" { "\\" } else { "/" };
    let filename = file.0.file_name().unwrap_or_default().to_str().unwrap_or_default();

    rsx! {
        div {
            class: "border rounded d-flex flex-row justify-content-between bg-body-tertiary",
            div {
                class: "d-flex flex-row gap-1",
                select {
                    class: "form-select border-0 rounded-0 rounded-start border-end",
                    style: "width: 110px",
                    onchange: move |e: FormEvent| {
                        let f_type: Option<PackageMemberType> = PackageMemberType::from_string(e.value().as_ref());

                        if let Some(t) = f_type {
                            on_type_change((p_buff.as_ref().clone(), t));
                        };
                    },

                    for member_type in PackageMemberType::iter() {
                        option {
                            "selected": member_type == file.1,
                            { let opt: String = member_type.into(); opt }
                        }
                    }

                }

                p {
                    class: "my-auto mx-2",
                    span {
                        class: "text-secondary",
                        { directory }
                    }
                    { filename }
                }
            }

            Button {
                class: "m-2",
                onclick: move |_| { on_delete(p_buff_2.as_ref().clone().clone()) },
                color: ButtonColor::Outline(Box::new(ButtonColor::Danger)),
                disabled: false,
                i {
                    class: "bi bi-trash",
                }
            }
        }
    }
}
