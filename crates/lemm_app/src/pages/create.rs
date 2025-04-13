use std::path::PathBuf;
use dioxus::html::a::class;
use crate::components::{Container, FileListItem};
use dioxus::prelude::*;
use directories::UserDirs;
use lib_lemm::data::PackageMemberType;
#[component]
pub fn Create() -> Element {
    let user_dirs = UserDirs::new().unwrap();
    let docs = PathBuf::from(user_dirs.document_dir().unwrap());
    let video = PathBuf::from(user_dirs.video_dir().unwrap());

    rsx! {
        Container {
            h1 { "Create" }
            p { "This is the create page." }
            br {}
            p { "You can create packages here." }

            Container {
                class: "d-flex flex-column gap-2",
                FileListItem {
                    file: (docs, PackageMemberType::TEXTURE),
                    on_delete: move |path| {
                        // Handle delete
                    },
                    on_type_change: move |(path, file_type)| {
                        // Handle type change
                    }
                }

                FileListItem {
                    file: (video, PackageMemberType::INI),
                    on_delete: move |path| {
                        // Handle delete
                    },
                    on_type_change: move |(path, file_type)| {
                        // Handle type change
                    }
                }
            }
        }
    }
}
