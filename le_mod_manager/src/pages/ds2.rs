use crate::components::{Button, Container, ModListItem};
use crate::data::{AppState, ButtonColor, ComponentSizing};
use crate::pages::state::DS2State;
use crate::pages::{CreateState, ToastManager, ToastType};
use dioxus::html::a::class;
use dioxus::html::completions::CompleteWithBraces::{header, progress};
use dioxus::html::link::disabled;
use dioxus::prelude::*;
use lib_lemm::data::{to_ascii_array, PackageHeader};

#[component]
pub fn DS2() -> Element {
    let state = use_context::<DS2State>();
    let load_order = state.load_order;
    let mod_options = state.enabled_mods;
    let progress= *state.progress.read();
    let is_installing = state.installing;

    use_effect(move || {
        let p = *state.progress.read();
        if p == Some(100u64) {
            use_context::<DS2State>().progress.set(None);
            use_context::<ToastManager>().add("Data written successfully!".to_string(), ToastType::Success);
        }
    });

    const IMG: Asset = asset!("assets/ds2.jpg");

    rsx! {
        Container { class: "d-flex h-100 position-fixed",
            div { class: "p-2 overflow-y-auto", width: "40%",
                div { class: "card",
                    img {
                        class: "card-img-top",
                        src: IMG,
                        alt: "Dark Souls II's Majula",
                    }

                    div { class: "card-body",
                        div { class: "card-title",
                            h2 { "Manage Your DS2LE Mods" }
                        }

                        div { class: "card-text",
                            p {
                                "This is the DS2 Lighting engine page. You can manage your DS2LE mods here."
                            }
                            p { "Mods will be installed in the order they are listed (top to bottom)." }
                            p {
                                "This means that the first mod in the list (top) will be installed first,
                                and anything that comes after it will be installed after it, overwriting any
                                shared files."
                            }
                        }

                        div { class: "d-flex gap-2 flex-wrap",
                            Button {
                                class: "flex-fill",
                                onclick: move |_| async move {
                                    use_context::<DS2State>().pick_archives().await;
                                },
                                "Add to list..."
                                i { class: "bi bi-box-arrow-up-right ms-2" }
                            }


                            Button {
                                class: "flex-fill",
                                color: ButtonColor::Primary,
                                onclick: |_| {
                                    use_context::<DS2State>().write();
                                },
                                disabled: load_order.is_empty(),
                                "Save list..."
                                i { class: "bi bi-floppy ms-2" }
                            }

                            Button {
                                class: "flex-fill",
                                color: ButtonColor::Success,
                                onclick: |_| {
                                    use_context::<DS2State>().write();
                                    use_context::<DS2State>().install();
                                },
                                disabled: load_order.is_empty() || is_installing(),
                                "Save & apply to game..."
                                i { class: "bi bi-stars ms-2" }
                            }
                        }

                        if progress.is_some() {
                        div {
                            class: "progress mt-3",
                            role: "progressbar",
                            div {
                                class: "progress-bar progress-bar-striped progress-bar-animated",
                                style: "width: ".to_string() + progress.unwrap().to_string().as_str() + "%;",
                               { "Writing... ".to_string() + progress.unwrap().to_string().as_str() + "%"}
                            }
                        }
                    }
                    }
                }
                br {}
                br {}
            }

            div {
                class: "overflow-y-auto d-flex h-100 flex-column gap-3 p-2",
                width: "60%",
                for (index , item) in load_order.iter().enumerate() {
                    ModListItem {
                        key: "mod_{index}",
                        index: index as u32,
                        rnd_id: item.1,
                        total: load_order.len() as u32,
                        mod_archive: item.0.clone(),
                        on_increase_order: move |idx| async move { use_context::<DS2State>().increase_mod_order(idx).await },
                        on_decrease_order: move |idx| async move { use_context::<DS2State>().decrease_mod_order(idx).await },
                        on_remove: move |idx| async move { use_context::<DS2State>().remove_mod(idx).await },
                        on_toggled: move |id| async move { use_context::<DS2State>().toggle_mod(id).await },
                        toggled: mod_options()[&item.1],
                    }
                }
                br {}
                br {}
            }
        }
    }
}
