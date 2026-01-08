// SPDX-License-Identifier: GPL-3.0-only

//! Frontend: Panel icon (reactive to trash status)
//!
//! Adaptive UI: uses colored icon in Dock and symbolic icon in Panel

use crate::app::Message;
use crate::trash_status::TrashStatus;
use cosmic::applet::PanelType;
use cosmic::prelude::*;
use cosmic::widget;

#[must_use]
pub fn view<'a>(trash_status: &TrashStatus, core: &cosmic::Core) -> Element<'a, Message> {
    match &core.applet.panel_type {
        PanelType::Dock => {
            // Dock: large colored icon (without -symbolic suffix)
            let icon_size = core.applet.suggested_size(false).0; // false = not symbolic
            let padding = core.applet.suggested_padding(false);

            widget::button::custom(
                widget::icon::from_name(trash_status.icon_name_dock()).size(icon_size),
            )
            .padding([padding.0, padding.1])
            .on_press(Message::TogglePopup)
            .class(cosmic::theme::Button::AppletIcon)
            .into()
        }
        PanelType::Panel | PanelType::Other(_) => {
            // Panel: small symbolic icon (icon_button adds -symbolic automatically)
            core.applet
                .icon_button(trash_status.icon_name_panel())
                .on_press(Message::TogglePopup)
                .into()
        }
    }
}
