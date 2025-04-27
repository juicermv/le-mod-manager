use dioxus::prelude::*;

use once_cell::sync::Lazy;
use std::{
	path::PathBuf,
	sync::atomic::{AtomicU64, Ordering},
};
use anyhow::Error;
use tokio::task::JoinError;

static TASKS_DONE: Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(0));
static TASKS_STARTED: Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(0));


/// Call this on the client to kick off the export.
/// It runs in the background, updating EXPORT_PROGRESS as it goes.
#[server]
pub async fn export_archive_server(
    files: Vec<(String, String)>,
    mod_name: String,
    mod_author: String,
    mod_version: String,
    output_path: String,
) -> Result<(), ServerFnError> {
    // Reset progress
    TASKS_DONE.store(0, Ordering::SeqCst);
    TASKS_STARTED.store(0, Ordering::SeqCst);
    
    // Do the blocking work on a blocking thread pool
    tokio::task::spawn_blocking(move || {
        use lib_lemm::data::{ModArchive, PackageMemberType};

        let out = PathBuf::from(output_path);
        match ModArchive::create(&out, mod_name, mod_author, mod_version) {
            Ok(archive) => {
                let total = files.len();
                for (i, (path_str, f_type)) in files.into_iter().enumerate() {
                    let mut archive = archive.clone();
                    TASKS_STARTED.fetch_add(1, Ordering::SeqCst);
                    tokio::task::spawn(async move {
                        match archive.add_file(
                            PathBuf::from(path_str),
                            PackageMemberType::from_string(&f_type).unwrap(),
                        ) {
                            Ok(_) => {
                                TASKS_DONE.fetch_add(1, Ordering::SeqCst);
                                Ok(())
                            }

                            Err(e) => {
                                Err(ServerFnError::new(e))
                            }
                        }
                    });
                }

                Ok(())
            }

            Err(e) => Err(ServerFnError::new(e)),
        }
    })
    .await
    .map_err(|e: JoinError| ServerFnError::new(e))?
}

/// Simple polling endpoint the UI can call to get the latest percent.
#[server]
pub async fn get_export_tasks_progress() -> Result<(u64, u64), ServerFnError> {
    Ok((TASKS_STARTED.load(Ordering::SeqCst), TASKS_DONE.load(Ordering::SeqCst)))
}
