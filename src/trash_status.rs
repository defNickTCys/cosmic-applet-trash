// SPDX-License-Identifier: GPL-3.0-only

//! Backend: Trash status monitoring
//!
//! Replicated from cosmic-files/src/tab.rs using trash-rs

use std::path::PathBuf;

/// Trash status (Backend, no UI dependencies)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrashStatus {
    pub is_empty: bool,
    pub item_count: usize,
}

impl TrashStatus {
    ///Checks current trash status using trash-rs
    ///
    /// Replicated from cosmic-files/src/tab.rs
    #[must_use]
    pub fn check() -> Self {
        let is_empty = trash::os_limited::is_empty().unwrap_or(true);
        let item_count = if is_empty {
            0
        } else {
            trash::os_limited::list()
                .map(|entries| entries.len())
                .unwrap_or(0)
        };

        Self {
            is_empty,
            item_count,
        }
    }

    /// Returns symbolic icon name for panel
    #[must_use]
    pub fn icon_name_panel(&self) -> &'static str {
        if self.is_empty {
            "user-trash-symbolic"
        } else {
            "user-trash-full-symbolic"
        }
    }

    /// Returns colored icon name for dock
    #[must_use]
    pub fn icon_name_dock(&self) -> &'static str {
        // In dock, system uses icons without "-symbolic" suffix
        // and automatically switches between empty/full
        if self.is_empty {
            "user-trash"
        } else {
            "user-trash-full"
        }
    }

    /// Returns default trash directory path on Linux
    #[allow(dead_code)] // Will be used in future phases
    #[must_use]
    pub fn trash_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(home).join(".local/share/Trash/files")
    }
}

impl Default for TrashStatus {
    fn default() -> Self {
        Self {
            is_empty: true,
            item_count: 0,
        }
    }
}
