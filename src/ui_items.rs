// SPDX-License-Identifier: GPL-3.0-only

//! UI Module: Trash Items List
//!
//! cosmic-files style: large icons, name+size column, centered actions

use crate::app::Message;
use crate::mime_icon::mime_icon;
use crate::trash_item_metadata::EnrichedTrashItem;
use cosmic::iced::Length;
use cosmic::prelude::*;
use cosmic::widget::{self, icon, scrollable, tooltip};

/// Renders scrollable list of trash items
///
/// NO title/divider here - those are in ui_popup.rs
#[must_use]
pub fn view(items: &[EnrichedTrashItem], sort_ascending: bool) -> Element<'_, Message> {
    if items.is_empty() {
        return widget::column().into();
    }

    // Triangle rotation based on sort order (cosmic-files pattern)
    let sort_icon = if sort_ascending {
        "pan-up-symbolic" // ▲ Ascending A-Z
    } else {
        "pan-down-symbolic" // ▼ Descending Z-A
    };

    let header = widget::row()
        .push(
            widget::button::custom(
                widget::row()
                    .push(widget::text::heading("Files"))
                    .push(widget::icon::from_name(sort_icon).size(16))
                    .spacing(4)
                    .align_y(cosmic::iced::Alignment::End),
            )
            .on_press(Message::ToggleSortOrder)
            .class(cosmic::theme::Button::MenuRoot),
        )
        .push(widget::horizontal_space())
        .push(widget::text::heading("Actions").width(Length::Fixed(80.0)))
        .spacing(12)
        .padding([0, 12])
        .align_y(cosmic::iced::Alignment::Center);

    // Items with dividers
    let mut item_list = Vec::new();
    for (i, item) in items.iter().enumerate() {
        item_list.push(item_row(item));
        if i < items.len() - 1 {
            item_list.push(widget::divider::horizontal::default().into());
        }
    }

    // Return: header + scrollable items (divider now in ui_popup)
    widget::column()
        .push(header)
        .push(widget::divider::horizontal::default())
        .push(
            scrollable(widget::column::with_children(item_list))
                .height(Length::Fixed(250.0)) // Max 250px, then scroll
                .width(Length::Fill),
        )
        .into()
}

/// Single item: Icon (32px) | Name+Size column | Actions
fn item_row(enriched: &EnrichedTrashItem) -> Element<'_, Message> {
    // Icon: 32px (smaller than before)
    let icon_handle = mime_icon(enriched.mime.clone(), 32);
    let icon_widget = icon::icon(icon_handle).size(32);

    // Text column: Name + Size
    let text_column = widget::column()
        .push(widget::text::body(
            enriched.item.name.to_string_lossy().to_string(),
        ))
        .push(widget::text::caption(&enriched.size_display))
        .spacing(4)
        .width(Length::Fill);

    // Actions (centered)
    let actions = widget::row()
        .push(tooltip(
            widget::button::icon(widget::icon::from_name("edit-undo-symbolic").size(16))
                .on_press(Message::RestoreItem(enriched.clone())),
            "Restore",
            tooltip::Position::Bottom,
        ))
        .push(tooltip(
            widget::button::icon(widget::icon::from_name("edit-delete-symbolic").size(16))
                .on_press(Message::DeleteItem(enriched.clone())),
            "Delete",
            tooltip::Position::Bottom,
        ))
        .spacing(4)
        .width(Length::Fixed(80.0));

    widget::row()
        .push(icon_widget)
        .push(text_column)
        .push(actions)
        .spacing(12)
        .padding([8, 12])
        .align_y(cosmic::iced::Alignment::Center)
        .into()
}
