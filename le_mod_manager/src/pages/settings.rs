use crate::components::{Button, DirectoryInput};
use crate::data::{ButtonColor, Padding, StepUnit};
use crate::{components::Container, pages::state::SettingsState};
use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    let state = use_context::<SettingsState>();
    let ds2_path = state.ds2_path;
    let ds2_path_valid = state.ds2_path_valid;
    let has_saved = state.has_saved;
    rsx! {
        Container {
            padding: Padding::all(StepUnit::Positive(3)),

            h2 { "Settings" }
            Container {
                DirectoryInput {
                    value: ds2_path,
                    label: "DS2 Game Directory Path",
                    is_valid: ds2_path_valid(),
                    onchange: move |e: FormEvent| async move {
                        let value: String = e.value();
                        use_context::<SettingsState>().try_set_ds2_path(value).await;
                    },
                    onpickerclicked: move |_| async move {
                        use_context::<SettingsState>().pick_ds2_path().await
                    }
                }

                Button {
                    class: "mt-4",
                    color: {
                        if has_saved()
                        {
                            ButtonColor::Outline(Box::new(ButtonColor::Secondary))}
                        else
                        {
                            ButtonColor::Success
                        }
                    },
                    onclick: move |_| async move {
                        use_context::<SettingsState>().write().await;
                    },

                    disabled: !ds2_path_valid(),
                    "Save"
                    i {
                        class: "bi bi-floppy ms-2",
                    }
                }
            }

        }
    }
}
