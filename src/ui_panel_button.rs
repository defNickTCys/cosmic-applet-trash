// SPDX-License-Identifier: GPL-3.0-only

//! Frontend: Ícone do painel (reativo ao status da lixeira)
//! 
//! UI adaptável: usa ícone colorido na Dock e symbolic no Painel

use crate::app::Message;
use crate::trash_status::TrashStatus;
use cosmic::applet::PanelType;
use cosmic::prelude::*;
use cosmic::widget;

pub fn view<'a>(trash_status: &TrashStatus, core: &cosmic::Core) -> Element<'a, Message> {
    match &core.applet.panel_type {
        PanelType::Dock => {
            // Dock: ícone grande colorido (sem suffix -symbolic)
            let icon_size = core.applet.suggested_size(false).0; // false = não symbolic
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
            // Painel: ícone pequeno symbolic (icon_button adiciona -symbolic automaticamente)
            core.applet
                .icon_button(trash_status.icon_name_panel())
                .on_press(Message::TogglePopup)
                .into()
        }
    }
}
