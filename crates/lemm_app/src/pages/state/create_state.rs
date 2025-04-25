use dioxus::prelude::*;
use lib_lemm::data::{ModArchive, PackageMemberType};
use rfd::AsyncFileDialog;
use std::collections::HashMap;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use async_std::task;
use crate::pages::{ToastManager, ToastType};

#[derive(PartialEq, Debug, Clone)]
pub struct CreateState {
    pub files: Signal<HashMap<PathBuf, PackageMemberType>>,
    pub filter: Signal<Option<PackageMemberType>>,
    pub mod_name: Signal<String>,
    pub mod_author: Signal<String>,
    pub mod_version: Signal<String>,

    pub progress: Signal<Option<u64>>
}

impl CreateState {
    pub fn new() -> Self {
        Self {
            files: Signal::new(HashMap::new()),
            filter: Signal::new(None),
            mod_name: Signal::new(String::new()),
            mod_author: Signal::new(String::new()),
            mod_version: Signal::new(String::new()),
            progress: Signal::new(None),
        }
    }

    pub fn set_mod_name(&mut self, mod_name: String) {
        self.mod_name.set(mod_name);
    }

    pub fn set_mod_author(&mut self, mod_author: String) {
        self.mod_author.set(mod_author);
    }

    pub fn set_mod_version(&mut self, mod_version: String) {
        self.mod_version.set(mod_version);
    }

    pub async fn pick_files(&mut self) {
        let dialog = AsyncFileDialog::new()
            .add_filter("LE Files", &["pkg", "ini", "dds", "cfg", "cfgpbr"])
            .set_title("Add files to mod archive...");

        let result = dialog.pick_files().await;
        if let Some (files) = result {
            let mut added_files = self.files.read().clone();
            let mut counter = 0;

            for file in files {
                let f_type: PackageMemberType = match file.path().extension() {
                    Some(ext) => {
                        match ext.to_str() {
                            Some(extension) => {
                                match extension {
                                    "pkg" => PackageMemberType::PKG,
                                    "ini" => PackageMemberType::INI,
                                    "dds" => PackageMemberType::TEXTURE,
                                    "cfg" => PackageMemberType::CONFIG,
                                    "cfgpbr" => PackageMemberType::CFGPBR,
                                    _ => continue
                                }
                            }

                            None => { continue }
                        }
                    }

                    None => {
                        continue;
                    }
                };

                counter += 1;
                added_files.insert(file.path().to_owned(), f_type);
            }

            self.files.set(added_files);
            use_context::<ToastManager>().add(format!("Successfully added {counter} files!"), ToastType::Success);
        }
    }

    pub async fn pick_engine_textures(&mut self) {
        let dialog = AsyncFileDialog::new()
            .add_filter("LE Engine Texture", &["dds"])
            .set_title("Add Engine Textures to mod archive...");

        let result = dialog.pick_files().await;
        if let Some (files) = result {
            let mut added_files = self.files.read().clone();
            let mut counter = 0;

            for file in files {
                counter += 1;
                added_files.insert(file.path().to_owned(), PackageMemberType::ETexture);
            }

            self.files.set(added_files);
            use_context::<ToastManager>().add(format!("Successfully added {counter} files!"), ToastType::Success);
        }
    }

    pub async fn update_file_type(&mut self, item: &PathBuf, f_type: PackageMemberType) {
        let mut files = self.files.read().clone();


        self.files.set(HashMap::new());
        task::sleep(Duration::from_millis(50)).await; // Fix a small bug that is out of my control

        if files.contains_key(item) {
            files.insert(item.clone(), f_type);
        }
        self.files.set(files);
    }

    pub fn update_filter(&mut self, filter: Option<PackageMemberType>) {
        self.filter.set(filter);
    }

    pub fn remove_file(&mut self, item: &PathBuf) {
        let mut files = self.files.read().clone();
        files.remove(item);
        self.files.set(files);
    }

    pub async fn export_archive(&mut self) {
        let dialog = AsyncFileDialog::new().set_title("Export your mod...").add_filter("Mod Archive", &["lemm"]);
        let result = dialog.save_file().await;
        if let Some(path) = result {
            match ModArchive::create(path.path(), self.mod_name.read().clone(), self.mod_author.read().clone(), self.mod_version.read().clone()) {
                Ok(mut archive) => {
                    let files = self.files.read().clone();
                    let total = files.len();
                    let mut current = 0usize;
                    for (path, f_type) in  files{
                        self.progress.set(Some(
                            ((current/total)*100) as u64
                        ));
                        match archive.add_file(path, f_type) { 
                            Ok(_) => {
                                current +=1;
                            }
                            
                            Err(e) => {
                                use_context::<ToastManager>().add(format!("Error writing file to archive! {e}"), ToastType::Error);
                                return;
                            }
                        }
                        
                    }

                    use_context::<ToastManager>().add("Archive created successfully!".into(), ToastType::Success);
                }

                Err(e) => {
                    use_context::<ToastManager>().add(format!("Could not create mod archive! {e}"), ToastType::Error);
                }
            }

        }
    }
}
