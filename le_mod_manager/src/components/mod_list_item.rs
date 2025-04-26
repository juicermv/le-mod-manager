use crate::components::Button;
use crate::data::ButtonColor;
use dioxus::prelude::*;
use lib_lemm::data::{from_ascii_array, ModArchive};


#[derive(PartialEq, Props, Clone)]
pub struct ModListItemProps {
    index: u32,
    total: u32,
    rnd_id: u32,
    mod_archive: ModArchive,
    toggled: bool,
    on_increase_order: EventHandler<u32>,
    on_decrease_order: EventHandler<u32>,
    on_remove: EventHandler<u32>,
    on_toggled: EventHandler<u32>,
}

pub fn ModListItem(
    props: ModListItemProps,
    // TODO: Implement ability to toggle which type of asset we want to include from the mod  (textures, pkgs, etc)
) -> Element {
    rsx! {
        div {
            class: "card",
            div {
                class: "card-header",
                { (props.index + 1).to_string() }
            }

            div {
                class: "card-body",
                div {
                    class: "card-title",
                    h3 {
                        { format!("{} {}",
                            props.mod_archive.get_mod_name(),
                            props.mod_archive.get_mod_version())
                        }
                    }
                }
                div {
                    class: "card-text",
                    p {
                        {
                            format!("Author: {}",
                                props.mod_archive.get_mod_author())
                        }
                    }
                }

                div {
                    class: "d-flex justify-content-between",
                    div {
                        Button {
                            class: "me-2",
                            color: if props.toggled {
                                ButtonColor::Success
                            } else {
                                ButtonColor::Outline(Box::new(ButtonColor::Danger))
                            },

                            onclick: move |_| props.on_toggled.call(props.rnd_id),
                            if props.toggled {
                                "Enabled"
                            } else {
                                "Disabled"
                            }
                        }

                        div {
                            class: "btn-group",
                            Button {
                                onclick: move |_| props.on_increase_order.call(props.index),
                                disabled: props.index == props.total - 1,
                                i {
                                    class: "bi bi-caret-down-fill",
                                }
                            }

                            Button {
                                onclick: move |_| props.on_decrease_order.call(props.index),
                                disabled: props.index == 0,
                                i {
                                    class: "bi bi-caret-up-fill",
                                }
                            }
                        }
                    }


                    Button {
                        onclick: move |_| props.on_remove.call(props.index),
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
}
