use crate::pages::{ToastManager, ToastType};
use crate::server::{export_archive_server, get_export_progress};
use async_std::task;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use flate2::read::GzEncoder;
use flate2::Compression;
use lib_lemm::data::{ModArchive, PackageMemberType};
use rfd::AsyncFileDialog;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use tokio::time::{sleep, Duration};

#[derive(PartialEq, Debug, Clone)]
pub struct CreateState {
    pub files: Signal<HashMap<PathBuf, PackageMemberType>>,
    pub filter: Signal<Option<PackageMemberType>>,
    pub mod_name: Signal<String>,
    pub mod_author: Signal<String>,
    pub mod_version: Signal<String>,

    pub progress: Signal<Option<u64>>,
    pub exporting: Signal<bool>,
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
            exporting: Signal::new(false),
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
        if let Some(files) = result {
            let mut added_files = self.files.read().clone();
            let mut counter = 0;

            for file in files {
                let f_type: PackageMemberType = match file.path().extension() {
                    Some(ext) => match ext.to_str() {
                        Some(extension) => match extension {
                            "pkg" => PackageMemberType::PKG,
                            "ini" => PackageMemberType::INI,
                            "dds" => PackageMemberType::TEXTURE,
                            "cfg" => PackageMemberType::CONFIG,
                            "cfgpbr" => PackageMemberType::CFGPBR,
                            _ => continue,
                        },

                        None => continue,
                    },

                    None => {
                        continue;
                    }
                };

                counter += 1;
                added_files.insert(file.path().to_owned(), f_type);
            }

            self.files.set(added_files);
            use_context::<ToastManager>().add(
                format!("Successfully added {counter} files!"),
                ToastType::Success,
            );
        }
    }

    pub async fn pick_engine_textures(&mut self) {
        let dialog = AsyncFileDialog::new()
            .add_filter("LE Engine Texture", &["dds"])
            .set_title("Add Engine Textures to mod archive...");

        let result = dialog.pick_files().await;
        if let Some(files) = result {
            let mut added_files = self.files.read().clone();
            let mut counter = 0;

            for file in files {
                counter += 1;
                added_files.insert(file.path().to_owned(), PackageMemberType::ETexture);
            }

            self.files.set(added_files);
            use_context::<ToastManager>().add(
                format!("Successfully added {counter} files!"),
                ToastType::Success,
            );
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

    async fn select_export_path() -> Option<PathBuf> {
        AsyncFileDialog::new()
            .add_filter("LEMM Archive", &["lemm"])
            .set_title("Export your mod...")
            .save_file()
            .await
            .map(|handle| PathBuf::from(handle.path()))
    }

    pub fn export_archive(&mut self) {
        let files_vec: Vec<(String, String)> = self
            .files
            .read()
            .clone()
            .into_iter()
            .map(|(p, t)| (p.to_string_lossy().into_owned(), t.into()))
            .collect();

        let data = serde_json::to_vec(&files_vec).unwrap();
        let mut compressed_data: Vec<u8> = Vec::new();
        GzEncoder::new(data.as_slice(), Compression::best())
            .read_to_end(&mut compressed_data)
            .unwrap();

        let name = self.mod_name.read().clone();
        let author = self.mod_author.read().clone();
        let version = self.mod_version.read().clone();
        let mut progress = self.progress;
        let mut exporting = self.exporting;

        // 1) kick off the export on the server
        spawn(async move {
            exporting.set(true);
            // prompt the user for a path
            if let Some(path) = Self::select_export_path().await {
                let output = path.to_string_lossy().into_owned();
                tracing::debug!("Exporting to {}", output);
                match export_archive_server(compressed_data, name, author, version, output).await {
                    Ok(_) => {
                        use_context::<ToastManager>().add(
                            "Mod archive written successfully!".to_string(),
                            ToastType::Success,
                        );
                    }

                    Err(e) => {
                        tracing::error!("{}", e);
                        /*use_context::<ToastManager>().add(
                            format!("Error while creating archive: {:?}", e),
                            ToastType::Success,
                        );*/
                    }
                }
            }
            exporting.set(false);
        });

        // 2) poll the progress endpoint
        spawn(async move {
            while *exporting.read() {
                if let Ok(pct) = get_export_progress().await {
                    progress.set(Some(pct));
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            // ensure 100% at the end
            progress.set(Some(100));
        });
    }
}
