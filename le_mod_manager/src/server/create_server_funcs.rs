use dioxus::prelude::*;
use lib_lemm::data::{ModArchive, PackageMemberType};
use once_cell::sync::Lazy;
use std::{
	path::PathBuf,
	sync::atomic::{AtomicU64, Ordering},
};
use std::io::Read;
use anyhow::Error;
use dioxus::logger::tracing;
use flate2::read::{GzDecoder, GzEncoder};
use tokio::task::JoinError;

// A global to hold progress [0..100]
static EXPORT_PROGRESS: Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(0));

/// Call this on the client to kick off the export.
/// It runs in the background, updating EXPORT_PROGRESS as it goes.
#[server(ExportArchive)]
pub async fn export_archive_server(
    files_serialized: Vec<u8>,
    mod_name: String,
    mod_author: String,
    mod_version: String,
    output_path: String,
) -> Result<(), ServerFnError> {
    // Reset progress
    EXPORT_PROGRESS.store(0, Ordering::SeqCst);
    tracing::debug!("Server exporting to {}", output_path);

    let mut decompressed_data: Vec<u8> = Vec::new();
    let mut decoder: GzDecoder<&[u8]> = GzDecoder::new(files_serialized.as_slice());
    decoder.read_to_end(&mut decompressed_data)?;

    let files: Vec<(String,String)> = serde_json::from_slice(decompressed_data.as_slice())?;

    // Do the blocking work on a blocking thread pool
    tokio::task::spawn_blocking(move || {
        let out = PathBuf::from(output_path);
        tracing::debug!("Writing to {}", out.display());
        match ModArchive::create(&out, mod_name, mod_author, mod_version) {
            Ok(archive) => {
                let mut archive = archive.clone();
                let total = files.len();
                for (i, (path_str, f_type)) in files.into_iter().enumerate() {
                    tracing::debug!("Processing file #{}\n\t{}\n\t{:?}", i, path_str, f_type);

                    match archive.add_file(
                        PathBuf::from(path_str),
                        PackageMemberType::from_string(&f_type).unwrap(),
                    ) {
                        Ok(_) => {
                            // Update progress [0..100]
                            let pct = ((i + 1) * 100 / total) as u64;
                            EXPORT_PROGRESS.store(pct, Ordering::SeqCst);
                        }

                        Err(e) => {
                            return Err(ServerFnError::new(e));
                        }
                    }

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
#[server(GetExportProgress)]
pub async fn get_export_progress() -> Result<u64, ServerFnError> {
    Ok(EXPORT_PROGRESS.load(Ordering::SeqCst))
}
