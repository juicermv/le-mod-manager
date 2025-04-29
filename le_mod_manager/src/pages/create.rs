use crate::components::{Button, Container, FileList};
use crate::data::{ButtonColor, Padding, StepUnit};
use crate::pages::{CreateState, ToastManager, ToastType};
use dioxus::html::completions::CompleteWithBraces::progress;
use dioxus::html::ol::start;
use dioxus::prelude::*;
use lib_lemm::data::PackageMemberType;
use std::path::PathBuf;

#[component]
pub fn Create() -> Element {
    let state = use_context::<CreateState>();
    let mut toast_manager= use_context::<ToastManager>();

    let mut progress = state.progress;
    let mod_name = state.mod_name;
    let mod_version = state.mod_version;
    let mod_author = state.mod_author;
    let state_files = state.files;
    let mut error = state.error;
    let filter = state.filter;
    let exporting = state.exporting;

    let mut can_export: bool = false;
    
    let mut files: Vec<(PathBuf, PackageMemberType)> = state_files()
        .iter()
        .map(|(key, val)| (key.clone(), *val))
        .collect();
    
    files.sort_by(|(a_path, a_type), (b_path, b_type)| {
        let a_str = String::from(a_path.file_name().unwrap().to_str().unwrap()).to_lowercase();
        let b_str = String::from(b_path.file_name().unwrap().to_str().unwrap()).to_lowercase();

        a_type.cmp(b_type).then(a_str.cmp(&b_str))
    });

    use_effect(move || {
        let p = progress();
        if p == Some(100u64) {
            progress.set(None);
            toast_manager.add(
                "Mod archive written successfully!".to_string(),
                ToastType::Success,
            );
        }

        if let Some(e) = error() {
            toast_manager.add(e, ToastType::Error);
            error.set(None);
        }
    });

    let files_empty =  files.is_empty();
    use_effect(move || {
        can_export = (!files_empty)
            && (!mod_author().is_empty())
            && (!mod_version().is_empty())
            && (!mod_name().is_empty());
    });

    rsx! {
        Container {
            padding: Padding::all(StepUnit::Positive(3)),

            h1 { "Create" }
            Container {
                h5 {
                    "1. Insert your mod's details:"
                }

                div {
                    div {
                        class: "input-group",
                        id: "details_input",
                        input {
                            class: "form-control",
                            placeholder: "Mod Name",
                            type: "text",
                            name: "mod_name",
                            maxlength: "32",
                            size: "32",
                            onchange: move |e| {
                                use_context::<CreateState>().set_mod_name(e.value())
                            }
                        }
                        input {
                            class: "form-control",
                            placeholder: "Author",
                            type: "text",
                            name: "mod_author",
                            maxlength: "16",
                            size: "16",
                            onchange: move |e| {
                                use_context::<CreateState>().set_mod_author(e.value())
                            }
                        }

                        input {
                            class: "form-control",
                            placeholder: "Version",
                            type: "text",
                            name: "mod_version",
                            maxlength: "5",
                            size: "5",
                            onchange: move |e| {
                                use_context::<CreateState>().set_mod_version(e.value())
                            }
                        }
                    }
                    div {
                        class: "form-text",
                        "Each field has a limited number of characters it accepts. 32 for the Mod Name field, 16 for the Author field, and 5 for the Version field."
                    }
                }

                h5 {
                    class: "mt-3",
                    "2. Add its contents:"
                }

                FileList {
                    files: files,
                    filter: filter(),
                    on_delete: move | path | use_context::<CreateState>().remove_file(&path),
                    on_type_change: move |(path, new_type): (PathBuf, PackageMemberType)| async move{ use_context::<CreateState>().update_file_type(&path, new_type).await },
                    on_filter_change: move |new_filter| use_context::<CreateState>().update_filter(new_filter),
                    on_add: move |_| async move { use_context::<CreateState>().pick_files().await },
                    on_add_engine_texture: move |_| async move { use_context::<CreateState>().pick_engine_textures().await }
                }

                div {
                    class: "d-flex flex-column flex-wrap gap-2 py-3",
                    h5 {
                        "3. Export your package"
                    }

                    Button {
                        color: ButtonColor::Success,
                        onclick: move |_| {
                            use_context::<CreateState>().export_archive()
                        },
                        disabled: !can_export || exporting(),
                        i {
                            class: "me-2 bi bi-box-seam"
                        }
                        "Export your package"
                    }

                    if progress().is_some() {
                        div {
                            class: "d-flex flex-row gap-2 flex-nowrap align-items-center",
                            label {
                               { "Writing... ".to_string() + progress.unwrap().to_string().as_str() + "%"}
                            }

                            div {
                                class: "progress flex-fill",
                                role: "progressbar",
                                div {
                                    class: "progress-bar progress-bar-striped progress-bar-animated",
                                    style: "width: ".to_string() + progress.unwrap().to_string().as_str() + "%;",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
