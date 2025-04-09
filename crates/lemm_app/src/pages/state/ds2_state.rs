use crate::data::{get_lemm_docs_dir, ModOptions, StoredLoadOrder, StoredLoadOrderItem};
use dioxus::prelude::*;
use lib_lemm::data::{ModArchive, PackageHeader};
use rand::RngCore;
use rfd::AsyncFileDialog;
use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(PartialEq, Clone)]
pub struct DS2State {
    pub load_order: Signal<Vec<(ModArchive, u32)>>,
    pub enabled_mods: Signal<HashMap<u32, bool>>,
}

impl DS2State {
    pub fn new() -> Self {
        Self {
            load_order: Signal::new(vec![]),
            enabled_mods: Signal::new(HashMap::new()),
        }
    }

    pub async fn increase_mod_order(&mut self, index: u32) {
        println!("Increase mod order: {}", index);
        let size = self.load_order.read().len();
        if size == 0 {
            return;
        };

        if (index as usize) < size - 1 {
            let mut list = self.load_order.read().clone();
            list.swap(index as usize, (index + 1) as usize);
            self.load_order.set(list);
        }
    }

    pub async fn decrease_mod_order(&mut self, index: u32) {
        println!("Decrease mod order: {}", index);
        if index > 0 {
            let mut list = self.load_order.read().clone();
            list.swap(index as usize, (index - 1) as usize);
            self.load_order.set(list);
        }
    }

    pub async fn remove_mod(&mut self, index: u32) {
        let size = self.load_order.read().len();
        if size == 0 {
            return;
        };

        if (index >= 0) && (index as usize) < size {
            let mut list = self.load_order.read().clone();
            let removed_val = &list.remove(index as usize);
            self.load_order.set(list);
            self.remove_mod_options(removed_val.1);
        }
    }

    pub async fn pick_archives(&mut self) {
        let dialogue = AsyncFileDialog::new()
            .add_filter("LE Mod Manager Archive", &["lemm"])
            .add_filter("All Files", &["*"])
            .set_title("Add archive to load order...");

        let result = dialogue.pick_files().await;
        match result {
            None => {}
            Some(files) => {
                let mut rng = rand::rng();
                let mut list = self.load_order.read().clone();
                let mut map = self.enabled_mods.read().clone();

                for file_handle in files {
                    let path = file_handle.path();
                    match ModArchive::open(path) {
                        Ok(archive) => {
                            let rnd_id = rng.next_u32();
                            map.insert(
                                rnd_id,
                                true,
                            );
                            list.push((archive, rnd_id));
                        }
                        Err(e) => {
                            println!("Error opening archive: {}", e);
                        }
                    }
                }
                self.enabled_mods.set(map);
                self.load_order.set(list);
            }
        }
    }

    pub async fn toggle_mod(&mut self, rnd_id: u32) {
        let mut map = self.enabled_mods.read().clone();
        map.insert(
            rnd_id,
            match map.get(&rnd_id) {
                None => true,
                Some(val) => !val,
            },
        );
        self.enabled_mods.set(map);
    }

    pub fn write(&self) -> Result<()>{
        let path = get_lemm_docs_dir()?;
        let file_path = path.join("ds2.toml");
        let load_order = self.load_order.read().clone();
        let enabled_mods = self.enabled_mods.read().clone();

        let load_order_to_store = StoredLoadOrder {
            mod_list: load_order.iter().map(|(archive, uid)| {
                let enabled = enabled_mods.get(uid).unwrap_or(&true);
                let path = archive.get_path();
                StoredLoadOrderItem {
                    path: path.to_str().unwrap_or_default().to_string(),
                    uid: uid.clone(),
                    enabled: enabled.clone(),
                }
            }).collect(),
        };

        match fs::write(&file_path, toml::to_string_pretty(&load_order_to_store)?) {
            Ok(_) => {
                println!("Wrote to file: {}", file_path.display());
                Ok(())
            }
            Err(e) => {
                println!("Error writing to file: {}", e);
                Err(e.into())
            }
        }
    }

    pub fn read() -> Self {
        match get_lemm_docs_dir() {
            Err(e) => {
                println!("Error getting docs dir: {}", e);
            }

            Ok(path) => {
                let file_path = path.join("ds2.toml");
                if file_path.exists() {
                    match fs::read_to_string(&file_path) {
                        Ok(contents) => {
                            match toml::from_str::<StoredLoadOrder>(&contents) {
                                Ok(load_order) => {
                                    let mut list = vec![];
                                    let mut map = HashMap::new();
                                    for item in load_order.mod_list {
                                        match ModArchive::open(item.path.clone()) {
                                            Ok(archive) => {
                                                list.push((archive, item.uid));
                                                map.insert(item.uid, item.enabled);
                                            }

                                            Err(e) => {
                                                println!("Error opening archive: {}", e);
                                            }
                                        }

                                    }

                                    return Self {
                                        load_order: Signal::new(list),
                                        enabled_mods: Signal::new(map),
                                    };
                                }
                                Err(e) => {
                                    println!("Error parsing file: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error reading file: {}", e);
                        }
                    }
                }
            }
        }

        Self::new()
    }

    fn remove_mod_options(&mut self, rnd_id: u32) {
        let mut map = self.enabled_mods.read().clone();
        match map.get(&rnd_id) {
            Some(_) => {
                map.remove(&rnd_id);
                self.enabled_mods.set(map);
            }
            None => {}
        }
    }
}
