// SPDX-License-Identifier: GPL-3.0-only

//! Frontend: Panel icon (reactive to trash status)
//!
//! Adaptive UI: uses colored icon in Dock and symbolic icon in Panel

use crate::app::Message;
use crate::trash_status::TrashStatus;
use cosmic::applet::PanelType;
use cosmic::iced::Length;
use cosmic::prelude::*;
use cosmic::widget;

#[must_use]
pub fn view<'a>(trash_status: &TrashStatus, core: &cosmic::Core) -> Element<'a, Message> {
    match &core.applet.panel_type {
        PanelType::Dock => {
            // Dock: large colored icon with proper button shape
            let icon_handle = widget::icon::from_name(trash_status.icon_name_dock()).handle();
            let suggested = core.applet.suggested_size(false); // false = not symbolic
            let (major_padding, minor_padding) = core.applet.suggested_padding(false);

            // Calculate padding based on orientation
            let (horizontal_padding, vertical_padding) = if core.applet.is_horizontal() {
                (major_padding, minor_padding)
            } else {
                (minor_padding, major_padding)
            };

            // Use layer_container pattern from cosmic-panel-button for correct hover shape
            let button = widget::button::custom(
                widget::layer_container(
                    widget::icon(icon_handle)
                        .width(Length::Fixed(suggested.0 as f32))
                        .height(Length::Fixed(suggested.1 as f32)),
                )
                .center(Length::Fill),
            )
            .width(Length::Fixed((suggested.0 + 2 * horizontal_padding) as f32))
            .height(Length::Fixed((suggested.1 + 2 * vertical_padding) as f32))
            .on_press_down(Message::TogglePopup)
            .class(cosmic::theme::Button::AppletIcon);

            // Wrap with tooltip
            core.applet
                .applet_tooltip(button, "Trash", false, Message::Surface, None)
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
