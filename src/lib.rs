// SPDX-License-Identifier: GPL-3.0-only

//! COSMIC Applet Trash Plus
//!
//! Advanced trash management with:
//! - Real-time trash status monitoring
//! - Drag & Drop for disk eject
//! - Drag & Drop for app uninstall (Flatpak/PackageKit)

pub mod app;
pub mod config;
pub mod file_manager;
pub mod i18n;
pub mod trash_status;
pub mod ui_panel_button;
pub mod ui_popup;
