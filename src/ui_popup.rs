// SPDX-License-Identifier: GPL-3.0-only

//! Frontend: Popup content

use crate::app::Message;
use crate::trash_item_metadata::EnrichedTrashItem;
use crate::trash_status::TrashStatus;
use crate::ui_items;
use cosmic::applet::{menu_button, padded_control};
use cosmic::iced::widget::{horizontal_rule, rule};
use cosmic::prelude::*;
use cosmic::widget::divider;
use cosmic::{theme, widget}; // Native divider

pub fn view<'a>(
    trash_status: &TrashStatus,
    trash_items: &'a [EnrichedTrashItem],
    sort_ascending: bool,
    _core: &cosmic::Core,
) -> Element<'a, Message> {
    let cosmic::cosmic_theme::Spacing {
        space_xxs, space_s, ..
    } = theme::active().cosmic().spacing;

    let mut content = if trash_status.is_empty {
        // Empty state: no header, start directly at buttons
        widget::column().padding([8, 0])
    } else {
        // Title: padded_control applies lateral padding automatically
        let title_row = widget::container(padded_control(
            widget::row()
                .push(widget::icon::from_name(trash_status.icon_name_dock()).size(32))
                .push(
                    widget::text::body("Trash")
                        .class(cosmic::theme::Text::Accent)
                        .size(24),
                )
                .push(widget::horizontal_space())
                .push(
                    widget::text::body(format!("{} items", trash_status.item_count))
                        .align_y(cosmic::iced::alignment::Vertical::Bottom),
                )
                .spacing(8)
                .align_y(cosmic::iced::Alignment::Center),
        ))
        .padding([8, 0]); // Only vertical padding

        // Accent divider with EXTERNAL padding
        let accent_divider = padded_control(horizontal_rule(1).class(cosmic::theme::Rule::Custom(
            Box::new(|theme| rule::Style {
                color: theme.cosmic().accent_color().into(),
                width: 1,
                radius: 0.0.into(),
                fill_mode: rule::FillMode::Full,
            }),
        )))
        .padding([0, space_s]);

        widget::column()
            .padding([8, 0])
            .push(title_row)
            .push(accent_divider)
            .push(padded_control(ui_items::view(trash_items, sort_ascending)))
            // Divider OUTSIDE items
            .push(padded_control(divider::horizontal::default()).padding([0, space_s]))
    };

    // Empty Trash button
    let empty_icon = if trash_status.is_empty {
        "user-trash-symbolic"
    } else {
        "user-trash-full-symbolic"
    };

    // Empty Trash button - conditional text
    let empty_text = if trash_status.is_empty {
        "Trash is empty"
    } else {
        "Empty trash..."
    };

    content = content
        .push(
            menu_button(
                widget::row()
                    .push(widget::icon::from_name(empty_icon).size(16))
                    .push(widget::text::body(empty_text))
                    .spacing(12)
                    .align_y(cosmic::iced::Alignment::Center),
            )
            .on_press_maybe(if trash_status.is_empty {
                None
            } else {
                Some(Message::EmptyTrash)
            }), // Conditional
        )
        .push(padded_control(divider::horizontal::default()).padding([space_xxs, space_s])) // Divider entre buttons
        .push(
            menu_button(
                widget::row()
                    .push(widget::icon::from_name("folder-open-symbolic").size(16))
                    .push(widget::text::body("Open trash in files..."))
                    .spacing(12)
                    .align_y(cosmic::iced::Alignment::Center),
            )
            .on_press(Message::OpenTrashFolder),
        );

    _core.applet.popup_container(content).into()
}
