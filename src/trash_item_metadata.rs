// SPDX-License-Identifier: GPL-3.0-only

//! Trash Item Metadata Module
//!
//! Pre-computes and caches metadata for trash items to avoid filesystem I/O during rendering.
//! Provides enriched items with size strings, MIME types, and sorted ordering (folders first).

use std::cmp::Ordering;

/// Enriched trash item with pre-computed metadata
///
/// Avoids filesystem access during UI rendering by caching:
/// - Formatted size string
/// - MIME type (for icon resolution)
/// - Is directory flag (for sorting)
#[derive(Debug, Clone)]
pub struct EnrichedTrashItem {
    /// Original trash item from trash-rs
    pub item: trash::TrashItem,
    /// Pre-formatted size string ("5.0 MB", "3 items", etc.)
    pub size_display: String,
    /// MIME type for icon resolution (uses cosmic-files cache)
    pub mime: mime_guess::Mime,
    /// Whether this item is a directory (for sorting)
    pub is_dir: bool,
}

impl EnrichedTrashItem {
    /// Creates enriched item with pre-computed metadata
    ///
    /// # Performance
    /// This performs I/O once during creation, not during every render.
    /// MIME type is detected but icon is resolved lazily via cached `mime_icon()`.
    #[must_use]
    pub fn from_trash_item(item: trash::TrashItem) -> Self {
        let (size_display, is_dir) = compute_size(&item);
        let mime = compute_mime(&item, is_dir);

        Self {
            item,
            size_display,
            mime,
            is_dir,
        }
    }

    /// Sorts items: folders first (alphabetical), then files (alphabetical)
    ///
    /// # Arguments
    /// * `ascending` - true for A-Z, false for Z-A (folders always stay first)
    pub fn sort_items(items: &mut [Self], ascending: bool) {
        items.sort_by(|a, b| {
            match (a.is_dir, b.is_dir) {
                (true, false) => Ordering::Less,    // Folders before files
                (false, true) => Ordering::Greater, // Files after folders
                _ => {
                    // Same type: alphabetical by name
                    let name_order = a
                        .item
                        .name
                        .to_string_lossy()
                        .to_lowercase()
                        .cmp(&b.item.name.to_string_lossy().to_lowercase());

                    if ascending {
                        name_order // A-Z
                    } else {
                        name_order.reverse() // Z-A
                    }
                }
            }
        });
    }
}

/// Computes size display string for trash item
///
/// Uses metadata.is_dir() for correct detection (not path.is_dir())
/// Works for ALL file types - icons handled by cosmic-files mime_icon()
#[allow(clippy::cast_precision_loss)]
fn compute_size(item: &trash::TrashItem) -> (String, bool) {
    let Ok(trash_folders) = trash::os_limited::trash_folders() else {
        return ("-".to_string(), false);
    };

    // Try ALL trash folders until we find the file
    let mut found_metadata = None;
    let mut found_path = None;
    for folder in trash_folders.iter() {
        let path = folder.join("files").join(&item.name);
        if let Ok(meta) = std::fs::metadata(&path) {
            found_metadata = Some(meta);
            found_path = Some(path);
            break;
        }
    }

    let Some(metadata) = found_metadata else {
        return ("-".to_string(), false);
    };

    if metadata.is_dir() {
        // Folders: count items
        let count = found_path
            .and_then(|p| std::fs::read_dir(p).ok())
            .map(std::iter::Iterator::count)
            .unwrap_or(0);
        (format!("{count} items"), true)
    } else {
        // Files (ALL types): format bytes
        let bytes = metadata.len();
        let size_str = if bytes == 0 {
            "0 B".to_string()
        } else if bytes < 1024 {
            format!("{bytes} B")
        } else if bytes < 1024 * 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        };
        (size_str, false)
    }
}

/// Computes MIME type for trash item (used with cosmic-files mime_icon cache)
///
/// Returns MIME type for efficient icon resolution via `mime_icon()` cache.
/// Directories return "inode/directory" MIME type.
fn compute_mime(item: &trash::TrashItem, is_dir: bool) -> mime_guess::Mime {
    if is_dir {
        return "inode/directory".parse().unwrap();
    }

    // Try ALL trash folders until we find the file
    let Ok(trash_folders) = trash::os_limited::trash_folders() else {
        return mime_guess::mime::TEXT_PLAIN;
    };

    for folder in trash_folders.iter() {
        let trash_path = folder.join("files").join(&item.name);
        if trash_path.exists() {
            // Use cosmic-files mime detection (handles metadata and content-based detection)
            return crate::mime_icon::mime_for_path(&trash_path, None, false);
        }
    }

    // Fallback if file not found in any folder
    mime_guess::mime::TEXT_PLAIN
}
