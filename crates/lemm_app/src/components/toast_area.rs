use crate::pages::ToastType;
use dioxus::prelude::*;

#[component]
pub fn ToastArea(toasts: Vec<(String, ToastType)>, on_remove: EventHandler<usize>) -> Element {
    rsx! {
        div {
            class: "toast-container position-fixed bottom-0 end-0 p-3",
            for (index, (message, toast_type)) in toasts.iter().enumerate() {
                div {
                    class: "toast fade show align-items-center",
                    role: "alert",
                    aria_live: "assertive",
                    aria_atomic: "true",
                    style: "z-index: 9999;",
                    div {
                        class: "toast-header",
                        i {
                            class: {
                                "bi ".to_string() + match toast_type {
                                    ToastType::Info => "bi-info text-info",
                                    ToastType::Success => "bi-stars text-success",
                                    ToastType::Warning => "bi-asterisk text-warning",
                                    ToastType::Error => "bi-heartbreak text-danger",
                                } + " me-2"
                            }
                        }

                        strong {
                            class: "me-auto",
                            {
                                toast_type.to_string()
                            }
                        }

                        button {
                            class: "btn-close",
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
