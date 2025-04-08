use dioxus::prelude::*;
use lib_lemm::data::{ModArchive, PackageHeader};
use rfd::AsyncFileDialog;

#[derive(PartialEq, Clone)]
pub struct InstallState {
    pub load_order: Signal<Vec<ModArchive>>,
}

impl InstallState {
    pub fn new() -> Self {
        Self {
            load_order: Signal::new(vec![]),
        }
    }

    pub fn increase_mod_order(&mut self, index: u32) {
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

    pub fn decrease_mod_order(&mut self, index: u32) {
        println!("Decrease mod order: {}", index);
        if index > 0 {
            let mut list = self.load_order.read().clone();
            list.swap(index as usize, (index - 1) as usize);
            self.load_order.set(list);
        }
    }

    pub fn remove_mod(&mut self, index: u32) {
        let size = self.load_order.read().len();
        if size == 0 {
            return;
        };

        if (index >= 0) && (index as usize) < size {
            let mut list = self.load_order.read().clone();
            list.remove(index as usize);
            self.load_order.set(list);
        }
    }

    pub async fn add_archive(&mut self) {
        let dialogue = AsyncFileDialog::new()
            .add_filter("LE Mod Manager Archive", &["lemm"])
            .add_filter("All Files", &["*"])
            .set_title("Add archive to load order...");

        let result = dialogue.pick_file().await;
        match result {
            None => {}
            Some(file_handle) => {
                let path = file_handle.path();
                match ModArchive::open(path) {
                    Ok(archive) => {
                        let mut list = self.load_order.read().clone();
                        list.push(archive);
                        self.load_order.set(list);
                    }
                    Err(e) => {
                        println!("Error opening archive: {}", e);
                    }
                }
            }
        }
    }
}
