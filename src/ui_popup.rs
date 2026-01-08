// SPDX-License-Identifier: GPL-3.0-only

//! Frontend: Popup content

use crate::app::Message;
use crate::fl;
use crate::trash_status::TrashStatus;
use cosmic::applet::{menu_button, padded_control};
use cosmic::prelude::*;
use cosmic::widget;

pub fn view<'a>(trash_status: &TrashStatus, core: &cosmic::Core) -> Element<'a, Message> {
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
        .push(padded_control(widget::divider::horizontal::default()))
        .push(
            menu_button(widget::text::body(fl!("open-trash"))).on_press(Message::OpenTrashFolder),
        );

    core.applet.popup_container(content).into()
}
