use crate::components::{Button, Container, ModListItem};
use crate::data::{AppState, ButtonColor, ComponentSizing};
use crate::pages::state::install_state::InstallState;
use dioxus::html::a::class;
use dioxus::html::completions::CompleteWithBraces::header;
use dioxus::html::link::disabled;
use dioxus::prelude::*;
use lib_lemm::data::{to_ascii_array, PackageHeader};

#[component]
pub fn Install() -> Element {
    let state = use_context::<InstallState>();
    let load_order = state.load_order;

    rsx! {
        Container {
            class: "d-flex",
            div {
                class: "p-2",
                width: "40%",
                div {
                    class: "card",
                    div {
                        class: "card-header",
                        h2 { "Manage Your Mods" }
                    }

                    div {
                        class: "card-body",
                        div {
                            class: "card-text",
                            p {
                                "This is the install page. You can manage your mods here."
                            }
                            p {
                                "Mods will be installed in the order they are listed (top to bottom)."
                            }
                            p {
                                "This means that the first mod in the list (top) will be installed first,
                                and anything that comes after it will be installed after it, overwriting any
                                shared files."
                            }
                        }

                        div {
                            class: "d-flex gap-2",
                            Button {
                                class: "btn btn-primary",
                                onclick: move |_| async move {
                                    use_context::<InstallState>().add_archive().await;
                                },
                                i {
                                    class: "bi bi-plus-lg",
                                }
                            }
                            Button {
                                class: "btn btn-primary",
                                color: ButtonColor::Success,
                                onclick: |_| {},
                                disabled: load_order.is_empty(),
                                "Install..."
                            }
                        }
                    }
                }
            }

            div {
                class: "overflow-y-scroll p-2",
                width: "60%",
                for (index, archive) in load_order.iter().enumerate()
                {
                    ModListItem {
                        key: "mod_{index}",
                        index: index as u32,
                        total: load_order.len() as u32,
                        mod_archive: archive.clone(),
                        on_increase_order: |idx| use_context::<InstallState>().increase_mod_order(idx),
                        on_decrease_order: |idx| use_context::<InstallState>().decrease_mod_order(idx),
                        on_remove: |idx| use_context::<InstallState>().remove_mod(idx),
                    }
                    br {}
                }
            }
        }
    }
}
