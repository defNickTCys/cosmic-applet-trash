// SPDX-License-Identifier: GPL-3.0-only

//! Integração nativa com cosmic-files
//!
//! Usa std::process::Command conforme padrão oficial COSMIC
//! Ref: cosmic-files/cosmic-files-applet/src/file_manager.rs

use std::process::Command;

/// Abre a lixeira no cosmic-files
///
/// Usa o argumento `--trash` que é suportado nativamente pelo cosmic-files
/// (ref: cosmic-files/src/lib.rs linha 123-124)
pub fn open_trash_folder() {
    match Command::new("cosmic-files").arg("--trash").spawn() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to open cosmic-files: {}", e);
        }
    }
}
