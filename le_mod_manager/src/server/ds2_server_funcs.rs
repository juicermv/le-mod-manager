use dioxus::prelude::*;
use lib_lemm::data::{
	ModArchive, PackageHeader, PackageMemberHeader, PackageMemberRef, PackageMemberType,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::{
	path::PathBuf,
	sync::atomic::{AtomicU64, Ordering},
};
use tokio::task::JoinError;
use std::fs;

static TASKS_STARTED: Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(0));
static TASKS_DONE: Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(0));


#[server]
pub async fn install_mods_server(
    archive_paths: Vec<String>,
    ds2_path: String,
) -> Result<(), ServerFnError> {
    // reset progress
    TASKS_DONE.store(0, Ordering::SeqCst);
    TASKS_STARTED.store(0, Ordering::SeqCst);

    // spawn_blocking so it doesn’t tie up the async threads
    let load_order = archive_paths
        .iter()
        .map(|path| ModArchive::open(path))
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect::<Vec<ModArchive>>();

    let ds2_path = PathBuf::from(ds2_path);

    // TODO add cfgpbr alphabetical shit
    tokio::task::spawn_blocking(move || {
        // Gather files from archives
        let mut refs: HashMap<PackageMemberRef, usize> = HashMap::new();
        for (index, archive) in load_order.iter().enumerate() {
            for file_ref in archive.get_refs() {
                refs.insert(file_ref, index);
            }
        }

        let total = refs.len();

        // Write files to disk
        for (i, (file_ref, index)) in refs.iter().enumerate() {
            TASKS_STARTED.fetch_add(1, Ordering::SeqCst);
            let write_path: PathBuf = ds2_path
                .join(match file_ref.package_member_type {
                    PackageMemberType::TEXTURE => "tex_override",
                    PackageMemberType::ETexture => "ds2le_atmosphere_presets/textures",
                    PackageMemberType::INI => "ds2le_atmosphere_presets",
                    PackageMemberType::CONFIG => "tex_override",
                    PackageMemberType::CFGPBR => "tex_override",
                    PackageMemberType::PKG => "ds2le_atmosphere_presets",
                })
                .join(&file_ref.name);

            let file_ref = file_ref.clone();
            let archive: ModArchive = load_order.get(*index).unwrap().clone(); // This shouldn't really throw an error
            tokio::task::spawn(async move {
                match archive.read_file_from_ref(&file_ref) {
                    Ok(contents) => {
                        fs::create_dir_all(write_path.parent().unwrap());
                        fs::write(write_path, contents.unwrap_or_default())?;

                        TASKS_DONE.fetch_add(1, Ordering::SeqCst);
                        Ok(())
                    }

                    Err(err) => {
                        Err(ServerFnError::new(err))
                    }
                }
            });
        }
        Ok(())
    })
    .await
    .map_err(|e: JoinError| ServerFnError::new(e))?
}

#[server]
pub async fn get_install_tasks_progress() -> Result<(u64, u64), ServerFnError> {
    Ok((TASKS_STARTED.load(Ordering::SeqCst), TASKS_DONE.load(Ordering::SeqCst)))
}
