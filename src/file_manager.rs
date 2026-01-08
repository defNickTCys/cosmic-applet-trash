// SPDX-License-Identifier: GPL-3.0-only

//! Native integration with cosmic-files
//!
//! Uses `std::process::Command` following official COSMIC pattern
//! Ref: cosmic-files/cosmic-files-applet/src/file_manager.rs

use std::process::Command;

/// Opens the trash folder in cosmic-files
///
/// Uses the `--trash` argument which is natively supported by cosmic-files
/// (ref: cosmic-files/src/lib.rs lines 123-124)
pub fn open_trash_folder() {
    match Command::new("cosmic-files").arg("--trash").spawn() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to open cosmic-files: {e}");
        }
    }
}
