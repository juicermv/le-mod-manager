use crate::pages::ToastType;
use dioxus::prelude::*;

#[component]
pub fn ToastArea(toasts: Vec<(String, ToastType)>, on_remove: EventHandler<usize>) -> Element {
    rsx! {
        div {
            class: "toast-container position-fixed bottom-0 end-0 p-3",
            for (index, (message, toast_type)) in toasts.iter().enumerate() {
                div {
                    class: { "toast fade show align-items-center ".to_string() + &toast_type.to_string() },
                    role: "alert",
                    aria_live: "assertive",
                    aria_atomic: "true",
                    style: "z-index: 9999;",
                    div {
                        class: "toast-header",
                        i {
                            class: {
                                "bi ".to_string() + match toast_type {
                                    ToastType::Info => "bi-info-lg",
                                    ToastType::Success => "bi-stars",
                                    ToastType::Warning => "bi-asterisk",
                                    ToastType::Error => "bi-exclamation-lg",
                                }
                            }
                        }

                        button {
                            class: "btn-close me-2 m-auto",
                            type: "button",
                            aria_label: "Close",
                            onclick: move |_| on_remove.call(index),
                        }
                    }

                    div {
                        class: "toast-body",
                        "{message}"
                    }
                }
            }
        }
    }
}
