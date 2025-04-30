use crate::components::Button;
use crate::data::ButtonColor;
use dioxus::prelude::*;
use lib_lemm::data::ModArchive;


#[component]
pub fn ModListItem(
    index: u32,
    total: u32,
    rnd_id: u32,
    mod_archive: ModArchive,
    toggled: bool,
    enabled: bool,
    on_increase_order: EventHandler<u32>,
    on_decrease_order: EventHandler<u32>,
    on_remove: EventHandler<u32>,
    on_toggled: EventHandler<u32>,
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
                        { format!("{} {}",
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
                    class: "d-flex justify-content-between",
                    div {
                        Button {
                            class: "me-2",
                            color: if toggled {
                                ButtonColor::Success
                            } else {
                                ButtonColor::Outline(Box::new(ButtonColor::Danger))
                            },
                            
                            disabled: !enabled,

                            onclick: move |_| on_toggled.call(rnd_id),
                            if toggled {
                                "Enabled"
                            } else {
                                "Disabled"
                            }
                        }

                        div {
                            class: "btn-group",
                            Button {
                                onclick: move |_| on_increase_order.call(index),
                                disabled: (index == total - 1) || !enabled,
                                i {
                                    class: "bi bi-caret-down-fill",
                                }
                            }

                            Button {
                                onclick: move |_| on_decrease_order.call(index),
                                disabled: (index == 0) || !enabled,
                                i {
                                    class: "bi bi-caret-up-fill",
                                }
                            }
                        }
                    }


                    Button {
                        onclick: move |_| on_remove.call(index),
                        color: ButtonColor::Outline(Box::new(ButtonColor::Danger)),
                        disabled: !enabled,
                        i {
                            class: "bi bi-trash",
                        }
                    }
                }
            }
        }
    }
}
