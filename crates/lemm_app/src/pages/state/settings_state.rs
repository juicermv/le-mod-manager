use crate::data::get_lemm_docs_dir;
use dioxus::prelude::{Readable, Signal, Writable};
use directories::UserDirs;
use serde::{Deserialize, Deserializer, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use toml::Table;

#[derive(PartialEq, Clone, Default)]
pub struct SettingsState {
    pub ds2_path: Signal<String>,
    pub ds2_path_valid: Signal<bool>,

    pub has_saved: Signal<bool>,
}

#[derive(PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct SettingsFile {
    pub ds2_path: String,
    pub ds2_path_valid: bool,
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            ds2_path: Signal::new(String::new()),
            ds2_path_valid: Signal::new(true),
            has_saved: Signal::new(false),
        }
    }

    // TODO: Error handling
    pub fn read() -> Self {
        match get_lemm_docs_dir() {
            Err(_) => Self::new(),
            Ok(path) => {
                let file_path = path.join("settings.toml");
                if !file_path.exists() {
                    return Self::new();
                }

                let contents = fs::read_to_string(file_path).unwrap_or_default();
                let settings: SettingsFile = toml::from_str(&contents).unwrap_or_default();

                Self {
                    ds2_path: Signal::new(settings.ds2_path),
                    ds2_path_valid: Signal::new(settings.ds2_path_valid),
                    has_saved: Signal::new(true),
                }
            }
        }
    }

    pub async fn try_set_ds2_path(&mut self, path: String) {
        match dunce::canonicalize(&path) {
            Err(_) => {
                self.ds2_path.set(path);
                self.ds2_path_valid.set(false);
            }

            Ok(path_buf) => {
                let is_dir = path_buf.is_dir();

                let includes_tex_override = path_buf.join("tex_override").exists()
                    && path_buf.join("tex_override").is_dir();

                let includes_ds2le_atmosphere_presets =
                    path_buf.join("ds2le_atmosphere_presets").exists()
                        && path_buf.join("ds2le_atmosphere_presets").is_dir();

                let includes_engine_textures = path_buf
                    .join("ds2le_atmosphere_presets")
                    .join("textures")
                    .exists()
                    && path_buf
                        .join("ds2le_atmosphere_presets")
                        .join("textures")
                        .is_dir();

                if is_dir
                    && includes_tex_override
                    && includes_ds2le_atmosphere_presets
                    && includes_engine_textures
                {
                    self.ds2_path
                        .set(path_buf.to_str().unwrap_or_default().to_string());
                    self.ds2_path_valid.set(true);
                    self.has_saved.set(false);
                } else {
                    self.ds2_path.set(path);
                    self.ds2_path_valid.set(false);
                }
            }
        }
    }

    pub async fn pick_ds2_path(&mut self) {
        let dialog = rfd::AsyncFileDialog::new().set_title("Pick DS2 Game folder.");
        let result = dialog.pick_folder().await;
        match result {
            None => {}
            Some(handle) => {
                let path = handle.path();
                self.try_set_ds2_path(path.to_str().unwrap_or_default().to_string())
                    .await;
            }
        }
    }

    // TODO: Error handling
    pub async fn write(&mut self) {
        if self.ds2_path_valid.read().clone() {
            match get_lemm_docs_dir() {
                Err(_) => {}
                Ok(path) => {
                    let file_path = path.join("settings.toml");
                    let mut settings = SettingsFile::default();
                    settings.ds2_path = self.ds2_path.read().clone();
                    settings.ds2_path_valid = self.ds2_path_valid.read().clone();

                    let toml_string = toml::to_string_pretty(&settings).unwrap_or_default();
                    fs::write(file_path, toml_string).unwrap_or_default();
                    self.has_saved.set(true);
                }
            }
        }
    }
}
