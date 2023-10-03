// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use super::{App, Message};
use cosmic::widget::{divider, grid, text};
use cosmic::Element;

impl App
where
    Self: cosmic::Application,
{
    pub fn view_typography(&self) -> Element<Message> {
        static SAMPLE_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";

        grid()
            .column_spacing(16)
            .row_spacing(32)
            // Header
            .push(text::heading("Text Style"))
            .push(text::heading("Font Size"))
            .push(text::heading("Line Height"))
            .push(text::heading("Weight"))
            .push(text::heading("Two-line Example"))
            .insert_row()
            .push_with(divider::horizontal::default(), |item| {
                item.width(5)
            })
            // Title 1
            .insert_row()
            .push(text::title1("Title 1"))
            .push(text::title1("32px"))
            .push(text::title1("44px"))
            .push(text::title1("Light (300)"))
            .push(text::title1(SAMPLE_TEXT).width(463))
            // Title 2
            .insert_row()
            .push(text::title2("Title 2"))
            .push(text::title2("28px"))
            .push(text::title2("36px"))
            .push(text::title2("Regular (400)"))
            .push(text::title2(SAMPLE_TEXT).width(376))
            // Title 3
            .insert_row()
            .push(text::title3("Title 3"))
            .push(text::title3("24px"))
            .push(text::title3("32px"))
            .push(text::title3("Regular (400)"))
            .push(text::title3(SAMPLE_TEXT).width(376))
            // Title 4
            .insert_row()
            .push(text::title4("Title 4"))
            .push(text::title4("20px"))
            .push(text::title4("28px"))
            .push(text::title4("Regular (400)"))
            .push(text::title4(SAMPLE_TEXT).width(335))
            // Heading
            .insert_row()
            .push(text::heading("Heading"))
            .push(text::heading("14px"))
            .push(text::heading("20px"))
            .push(text::heading("Semibold (600)"))
            .push(text::heading(SAMPLE_TEXT).width(234))
            // Caption heading
            .insert_row()
            .push(text::caption_heading("Caption Heading"))
            .push(text::caption_heading("10px"))
            .push(text::caption_heading("14px"))
            .push(text::caption_heading("Semibold (600)"))
            .push(text::caption_heading(SAMPLE_TEXT).width(164))
            // Body
            .insert_row()
            .push(text::body("Body"))
            .push(text::body("14px"))
            .push(text::body("20px"))
            .push(text::body("Regular (400)"))
            .push(text::body(SAMPLE_TEXT).width(234))
            // Caption
            .insert_row()
            .push(text::caption("Caption"))
            .push(text::caption("10px"))
            .push(text::caption("14px"))
            .push(text::caption("Regular (400)"))
            .push(text::caption(SAMPLE_TEXT).width(164))
            // Monotext
            .insert_row()
            .push(text::monotext("Monotext"))
            .push(text::monotext("14px"))
            .push(text::monotext("20px"))
            .push(text::monotext("Regular (400)"))
            .push(text::monotext(SAMPLE_TEXT).width(280))
            .into()
    }
}
