use crate::components::{Button, Container, FileList, FileListItem};
use crate::data::{ButtonColor, Padding, StepUnit};
use crate::pages::CreateState;
use dioxus::html::a::class;
use dioxus::prelude::*;
use directories::UserDirs;
use lib_lemm::data::PackageMemberType;
use std::path::PathBuf;
use dioxus::html::ol::start;

#[component]
pub fn Create() -> Element {
    let state = use_context::<CreateState>();

    /*let user_dirs = UserDirs::new().unwrap();
    let docs = PathBuf::from(user_dirs.document_dir().unwrap());
    let video = PathBuf::from(user_dirs.video_dir().unwrap());
    let files = vec![
        (video, PackageMemberType::TEXTURE),
        (docs, PackageMemberType::INI)
    ];*/

    let mut files: Vec<(PathBuf, PackageMemberType)> = state
        .files
        .read()
        .clone()
        .iter()
        .map(|(key, val)| (key.clone(), *val))
        .collect();

    // Alphabetical sort (hopefully)
    files.sort_by(|(a_path, a_type), (b_path, b_type)| {
        let a_str = String::from(a_path.file_name().unwrap().to_str().unwrap()).to_lowercase();
        let b_str = String::from(b_path.file_name().unwrap().to_str().unwrap()).to_lowercase();

        a_type.cmp(b_type).then(a_str.cmp(&b_str))
    });

    let mod_name_empty = state.mod_name.read().clone().is_empty();
    let mod_version_empty = state.mod_version.read().clone().is_empty();
    let mod_author_empty = state.mod_author.read().clone().is_empty();
    let progress = state.progress.read().clone();
    let can_export: bool = (!files.is_empty()) && (!mod_author_empty) && (!mod_version_empty) && (!mod_name_empty);


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
                            maxlength: "10",
                            size: "10",
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
                        "Each field has a limited number of characters it accepts. 32 for the Mod Name field, 10 for the Author field, and 5 for the Version field."
                    }
                }

                h5 {
                    class: "mt-3",
                    "2. Add its contents:"
                }

                FileList {
                    files: files,
                    filter: *state.filter.read(),
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
                        onclick: move |_| async move {
                            use_context::<CreateState>().export_archive().await
                        },
                        disabled: !can_export,
                        i {
                            class: "me-2 bi bi-box-seam"
                        }
                        "Export your package"
                    }

                    if progress.is_some() {
                        div {
                            class: "progress",
                            role: "progressbar",
                            div {
                                class: "progress-bar progress-bar-striped progress-bar-animated",
                                style: "width: ".to_string() + progress.unwrap().to_string().as_str() + "%;",
                               { "Writing...".to_string() + progress.unwrap().to_string().as_str() + "%;"}
                            }
                        }
                    }
                }
            }
        }
    }
}
