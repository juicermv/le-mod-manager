use std::path::PathBuf;
use anyhow::Result;
use directories::UserDirs;

pub fn get_lemm_docs_dir() -> Result<PathBuf> {
    match UserDirs::new() {
        None => Err(anyhow::anyhow!("User directories not found")),
        Some(user_dirs) => match user_dirs.document_dir() {
            None => Err(anyhow::anyhow!("Documents directory not found")),
            Some(documents_dir) => {
                let path = documents_dir.join("LE Mod Manager");
                match path.exists() {
                    true => match path.is_dir() {
                        true => Ok(path),
                        false => Err(anyhow::anyhow!("Path exists but is not a directory")),
                    },
                    false => {
                        std::fs::create_dir_all(&path)?;
                        Ok(path)
                    }
                }
            }
        },
    }


}