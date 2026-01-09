// SPDX-License-Identifier: GPL-3.0-only

//! Frontend: Popup content

use crate::app::Message;
use crate::fl;
use crate::trash_status::TrashStatus;
use cosmic::applet::{menu_button, padded_control};
use cosmic::prelude::*;
use cosmic::{theme, widget};

pub fn view<'a>(trash_status: &TrashStatus, core: &cosmic::Core) -> Element<'a, Message> {
    let cosmic::cosmic_theme::Spacing {
        space_xxs, space_s, ..
    } = theme::active().cosmic().spacing;
    let content = widget::column()
        .padding([8, 0])
        .push(
            widget::text::title4(fl!("trash"))
                .apply(widget::container)
                .padding([12, 20]),
        )
        .push(
            widget::text(format!("{} items", trash_status.item_count))
                .apply(widget::container)
                .padding([0, 20, 12, 20]),
        )
        .push(
            padded_control(widget::divider::horizontal::default())
                .padding([space_xxs, space_s]),
        );

    // Empty Trash button - adaptive state
    let empty_icon = if trash_status.is_empty {
        "user-trash-symbolic"
    } else {
        "user-trash-full-symbolic"
    };

    let empty_text = if trash_status.is_empty {
        "Trash is empty"
    } else {
        "Empty trash..."
    };

    let content = content
        .push(
            menu_button(
                widget::row()
                    .push(widget::icon::from_name(empty_icon).size(16).symbolic(true))
                    .push(widget::text::body(empty_text))
                    .spacing(12)
                    .align_y(cosmic::iced::Alignment::Center),
            )
            .on_press_maybe(if trash_status.is_empty {
                None
            } else {
                Some(Message::EmptyTrash)
            }),
        )
        .push(
            padded_control(widget::divider::horizontal::default())
                .padding([space_xxs, space_s]),
        )
        .push(
            menu_button(
                widget::row()
                    .push(
                        widget::icon::from_name("folder-symbolic")
                            .size(16)
                            .symbolic(true),
                    )
                    .push(widget::text::body("Open trash in files..."))
                    .spacing(12)
                    .align_y(cosmic::iced::Alignment::Center),
            )
            .on_press(Message::OpenTrashFolder),
        );

    core.applet.popup_container(content).into()
}
