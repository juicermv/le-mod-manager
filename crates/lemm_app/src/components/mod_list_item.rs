use crate::components::Button;
use crate::data::ButtonColor;
use dioxus::prelude::*;
use lib_lemm::data::{from_ascii_array, ModArchive};

#[component]
pub fn ModListItem(
    index: u32,
    total: u32,
    mod_archive: ModArchive,
    on_increase_order: EventHandler<u32>,
    on_decrease_order: EventHandler<u32>,
    on_remove: EventHandler<u32>,
    // TODO: Implement ability to toggle which type of asset we want to include from the mod  (textures, pkgs, etc)
) -> Element {
    rsx! {
        div {
            class: "card",
            div {
                class: "card-header",
                { (index + 1).to_string() }
            }

            div {
                class: "card-body",
                div {
                    class: "card-title",
                    h3 {
                        { format!("{} v{}",
                            mod_archive.get_mod_name(),
                            mod_archive.get_mod_version())
                        }
                    }
                }
                div {
                    class: "card-text",
                    p {
                        {
                            format!("Author: {}",
                                mod_archive.get_mod_author())
                        }
                    }
                }

                div {
                    class: "btn-group me-2",
                    Button {
                        onclick: move |_| on_increase_order(index),
                        disabled: index == total - 1,
                        i {
                            class: "bi bi-caret-down-fill",
                        }
                    }

                    Button {
                        onclick: move |_| on_decrease_order(index),
                        disabled: index == 0,
                        i {
                            class: "bi bi-caret-up-fill",
                        }
                    }
                }

                Button {
                    onclick: move |_| on_remove(index),
                    color: ButtonColor::Outline(Box::new(ButtonColor::Danger)),
                    disabled: false,
                    i {
                        class: "bi bi-trash",
                    }
                }
            }
        }
    }
}
