// SPDX-License-Identifier: GPL-3.0-only

//! Backend: Asynchronous trash operations
//!
//! Following cosmic-files patterns for non-blocking trash operations.
//! All trash-rs calls are wrapped in `spawn_blocking` to prevent blocking the async runtime.

use std::path::PathBuf;

/// Lists all items currently in the trash
///
/// # Errors
///
/// Returns error if trash listing fails (permissions, corrupted trash, etc.)
pub async fn list_items() -> Result<Vec<trash::TrashItem>, trash::Error> {
    tokio::task::spawn_blocking(trash::os_limited::list)
        .await
        .map_err(|e| {
            eprintln!("Failed to spawn list_items task: {e}");
            trash::Error::Unknown {
                description: format!("Task spawn failed: {e}"),
            }
        })?
}

/// Empties the entire trash (permanently deletes all items)
///
/// Operations run in background via `spawn_blocking`. Items are deleted sequentially.
///
/// # Errors
///
/// Returns error if:
/// - Listing trash items fails
/// - Any item deletion fails (partial failures collected and reported)
pub async fn empty_trash() -> Result<(), trash::Error> {
    tokio::task::spawn_blocking(|| {
        let items = trash::os_limited::list()?;
        let mut errors = Vec::new();

        for item in items {
            if let Err(e) = trash::os_limited::purge_all([item]) {
                errors.push(e);
            }
        }

        // Report partial failures
        if !errors.is_empty() {
            eprintln!("Failed to purge {} items during empty_trash", errors.len());
            for err in &errors {
                eprintln!("  - {err}");
            }
            return Err(trash::Error::Unknown {
                description: format!("Failed to delete {} items", errors.len()),
            });
        }

        Ok(())
    })
    .await
    .map_err(|e| {
        eprintln!("Failed to spawn empty_trash task: {e}");
        trash::Error::Unknown {
            description: format!("Task spawn failed: {e}"),
        }
    })?
}

/// Restores a trash item to its original location
///
/// # Errors
///
/// Returns error if:
/// - Original path no longer exists
/// - Permissions prevent restoration
/// - File conflicts at original location
pub async fn restore_item(item: trash::TrashItem) -> Result<PathBuf, trash::Error> {
    let original_path = item.original_path();

    tokio::task::spawn_blocking(move || {
        trash::os_limited::restore_all([item])?;
        Ok(original_path)
    })
    .await
    .map_err(|e| {
        eprintln!("Failed to spawn restore_item task: {e}");
        trash::Error::Unknown {
            description: format!("Task spawn failed: {e}"),
        }
    })?
}

/// Permanently deletes a trash item (cannot be undone)
///
/// # Errors
///
/// Returns error if deletion fails (permissions, locked file, etc.)
pub async fn delete_item(item: trash::TrashItem) -> Result<(), trash::Error> {
    tokio::task::spawn_blocking(move || trash::os_limited::purge_all([item]))
        .await
        .map_err(|e| {
            eprintln!("Failed to spawn delete_item task: {e}");
            trash::Error::Unknown {
                description: format!("Task spawn failed: {e}"),
            }
        })?
}
