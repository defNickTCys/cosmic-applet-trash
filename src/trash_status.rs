// SPDX-License-Identifier: GPL-3.0-only

//! Backend: Status da lixeira
//!
//! Replicado de cosmic-files/src/tab.rs usando trash-rs

use std::path::PathBuf;

/// Status da lixeira (Backend, sem dependências de UI)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrashStatus {
    pub is_empty: bool,
    pub item_count: usize,
}

impl TrashStatus {
    /// Verifica o status atual da lixeira usando trash-rs
    ///
    /// Replicado de cosmic-files/src/tab.rs
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

    /// Retorna o ícone symbolic para o painel
    pub fn icon_name_panel(&self) -> &'static str {
        if self.is_empty {
            "user-trash-symbolic"
        } else {
            "user-trash-full-symbolic"
        }
    }

    /// Retorna o ícone colorido para a dock
    pub fn icon_name_dock(&self) -> &'static str {
        // Na dock, o sistema usa os ícones sem "-symbolic"
        // e alterna automaticamente entre vazio/cheio
        if self.is_empty {
            "user-trash"
        } else {
            "user-trash-full"
        }
    }

    /// Retorna o caminho padrão da lixeira no Linux
    #[allow(dead_code)] // Será usado na Fase 1 para monitoramento
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
