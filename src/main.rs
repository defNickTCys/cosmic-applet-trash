// SPDX-License-Identifier: GPL-3.0-only

mod app;
mod config;
mod file_manager;
mod i18n;
mod trash_status;
mod ui_panel_button;
mod ui_popup;

fn main() -> cosmic::iced::Result {
    // Initialize i18n
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();
    i18n::init(&requested_languages);

    // Run applet
    cosmic::applet::run::<app::AppModel>(())
}
