// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use super::{App, Message};
use cosmic::iced_core::Alignment;
use cosmic::widget::{button, column, grid, icon, row, text};
use cosmic::Element;

impl App
where
    Self: cosmic::Application,
{
    pub fn view_buttons(&self) -> Element<Message> {
        column()
            .spacing(24)
            .push(text::title1("Text Buttons"))
            // Suggested button header
            .push(
                column()
                    .spacing(8)
                    .push(text::title3("Suggested Button"))
                    .push(text("Highest level of attention, there should only be one primary button used on the page.").size(14.0))
            )
            // Suggested button demo
            .push(
                row()
                    .spacing(36)
                    .push(button::suggested("Label").on_press(Message::ShowContext("Suggested Button")))
                    .push(button::suggested("Label").on_press(Message::ShowContext("Suggested Button")).leading_icon(self.leading_icon.clone()))
                    .push(button::suggested("Label").on_press(Message::ShowContext("Suggested Button")).trailing_icon(self.trailing_icon.clone()))
                    .push(button::suggested("Label").on_press(Message::ShowContext("Suggested Button")).leading_icon(self.app_icon.clone()))
                    .push(
                        button::suggested("Label")
                            .on_press(Message::ShowContext("Suggested Button"))
                            .leading_icon(self.app_icon.clone())
                            .trailing_icon(self.trailing_icon.clone())
                    )
                    .push(
                        button::suggested("Disabled")
                            .leading_icon(self.app_icon.clone())
                            .trailing_icon(self.trailing_icon.clone())
                    )
            )
            // Destructive button header
            .push(
                column()
                    .spacing(8)
                    .push(text::title3("Destructive Button"))
                    .push(text("Highest level of attention, there should only be one primary button used on the page.").size(14.0))
            )
            // Destructive button demo
            .push(
                row()
                    .spacing(36)
                    .push(button::destructive("Label").on_press(Message::ShowContext("Destructive Button")))
                    .push(button::destructive("Label").on_press(Message::ShowContext("Destructive Button")).leading_icon(self.leading_icon.clone()))
                    .push(button::destructive("Label").on_press(Message::ShowContext("Destructive Button")).trailing_icon(self.trailing_icon.clone()))
                    .push(button::destructive("Label").on_press(Message::ShowContext("Destructive Button")).leading_icon(self.app_icon.clone()))
                    .push(
                        button::destructive("Label")
                            .on_press(Message::ShowContext("Destructive Button"))
                            .leading_icon(self.app_icon.clone())
                            .trailing_icon(self.trailing_icon.clone())
                    )
                    .push(
                        button::destructive("Disabled")
                            .leading_icon(self.app_icon.clone())
                            .trailing_icon(self.trailing_icon.clone())
                    )
            )
            // Standard button header
            .push(
                column()
                    .spacing(8)
                    .push(text::title3("Standard Button"))
                    .push(
                        text(
                            "Requires less attention from the user. Could be more \
                            than one button on the page, if necessary."
                        )
                        .size(14.0)
                    )
            )
            // Standard button demo
            .push(
                row()
                    .spacing(36)
                    .push(button::standard("Label").on_press(Message::ShowContext("Standard Button")))
                    .push(button::standard("Label").on_press(Message::ShowContext("Standard Button")).leading_icon(self.leading_icon.clone()))
                    .push(button::standard("Label").on_press(Message::ShowContext("Standard Button")).trailing_icon(self.trailing_icon.clone()))
                    .push(button::standard("Label").on_press(Message::ShowContext("Standard Button")).leading_icon(self.app_icon.clone()))
                    .push(
                        button::standard("Label")
                            .on_press(Message::ShowContext("Standard Button"))
                            .leading_icon(self.app_icon.clone())
                            .trailing_icon(self.trailing_icon.clone())
                    )
                    .push(
                        button::standard("Disabled")
                            .leading_icon(self.app_icon.clone())
                            .trailing_icon(self.trailing_icon.clone())
                    )
            )
            // Text button header
            .push(
                column()
                    .spacing(8)
                    .push(text::title3("Text Button"))
                    .push(text(
                        "Lowest priority actions, especially when presenting multiple options. Because text buttons \
                        don’t have a visible container in their default state, they don’t distract from nearby \
                        content. But they are also more difficult to recognize because of that."
                    ).size(14.0))
            )
            // Text button demo
            .push(
                row()
                    .spacing(36)
                    .push(button::text("Label").on_press(Message::ShowContext("Text Button")))
                    .push(button::text("Label").on_press(Message::ShowContext("Text Button")).leading_icon(self.leading_icon.clone()))
                    .push(button::text("Label").on_press(Message::ShowContext("Text Button")).trailing_icon(self.trailing_icon.clone()))
                    .push(button::text("Label").on_press(Message::ShowContext("Text Button")).leading_icon(self.app_icon.clone()))
                    .push(
                        button::text("Label")
                            .on_press(Message::ShowContext("Text Button"))
                            .leading_icon(self.app_icon.clone())
                            .trailing_icon(self.trailing_icon.clone())
                    )
                    .push(
                        button::text("Disabled")
                            .leading_icon(self.app_icon.clone())
                            .trailing_icon(self.trailing_icon.clone())
                    )
            )
            // Icon buttons
            .push(text::title1("Icon Buttons"))
            .push(view_icon_buttons(self.bt_icon.clone(), "Icon Buttons"))
            .push(text::title1("App Icon Buttons"))
            .push(view_icon_buttons(self.app_icon.clone(), "App Icon Buttons"))
            .push(text::title1("Hyperlinks"))
            .push(text::body("All the buttons have Default, Hover, Pressed, and Disabled states. Buttons in any of the states can have a Focused indicator signifying the button is ready to interact."))
            .push(
                row()
                    .spacing(36)
                    .push(button::link("Hyperlink").on_press(Message::ShowContext("Hyperlink")))
                    .push(button::link("Hyperlink").trailing_icon(true).on_press(Message::ShowContext("Hyperlink")))
                    .push(button::link("Hyperlink"))
                    .push(button::link("Hyperlink").trailing_icon(true))
            )
            .into()
    }
}

fn view_icon_buttons(
    icon: icon::Handle,
    context_header: &'static str,
) -> impl Into<Element<'static, Message>> {
    grid()
        .column_alignment(Alignment::Center)
        .column_spacing(24)
        .row_alignment(Alignment::Center)
        .row_spacing(36)
        .push(
            button::icon(icon.clone())
                .extra_small()
                .on_press(Message::ShowContext(context_header))
                .tooltip("Extra small icon button"),
        )
        .push(
            button::icon(icon.clone())
                .extra_small()
                .on_press(Message::ShowContext(context_header))
                .tooltip("Extra small icon button")
                .label("Label"),
        )
        .push(
            button::icon(icon.clone())
                .extra_small()
                .tooltip("Extra small icon button")
                .label("Label"),
        )
        .push(
            button::icon(icon.clone())
                .extra_small()
                .on_press(Message::ShowContext(context_header))
                .tooltip("Extra small icon button")
                .label("Label")
                .vertical(true),
        )
        .push(
            button::icon(icon.clone())
                .extra_small()
                .tooltip("Extra small icon button")
                .label("Label")
                .vertical(true),
        )
        .insert_row()
        .push(
            button::icon(icon.clone())
                .on_press(Message::ShowContext(context_header))
                .tooltip("Small icon button"),
        )
        .push(
            button::icon(icon.clone())
                .on_press(Message::ShowContext(context_header))
                .tooltip("Small icon button")
                .label("Label"),
        )
        .push(
            button::icon(icon.clone())
                .tooltip("Small icon button")
                .label("Label"),
        )
        .push(
            button::icon(icon.clone())
                .on_press(Message::ShowContext(context_header))
                .tooltip("Small icon button")
                .label("Label")
                .vertical(true),
        )
        .push(
            button::icon(icon.clone())
                .tooltip("Small icon button")
                .label("Label")
                .vertical(true),
        )
        .insert_row()
        .push(
            button::icon(icon.clone())
                .medium()
                .on_press(Message::ShowContext(context_header))
                .tooltip("Medium icon button"),
        )
        .push(
            button::icon(icon.clone())
                .medium()
                .on_press(Message::ShowContext(context_header))
                .tooltip("Medium icon button")
                .label("Label"),
        )
        .push(
            button::icon(icon.clone())
                .medium()
                .tooltip("Medium icon button")
                .label("Label"),
        )
        .push(
            button::icon(icon.clone())
                .medium()
                .on_press(Message::ShowContext(context_header))
                .tooltip("Medium icon button")
                .label("Label")
                .vertical(true),
        )
        .push(
            button::icon(icon.clone())
                .medium()
                .tooltip("Medium icon button")
                .label("Label")
                .vertical(true),
        )
        .insert_row()
        .push(
            button::icon(icon.clone())
                .large()
                .on_press(Message::ShowContext(context_header))
                .tooltip("Large icon button"),
        )
        .push(
            button::icon(icon.clone())
                .large()
                .on_press(Message::ShowContext(context_header))
                .tooltip("Large icon button")
                .label("Label"),
        )
        .push(
            button::icon(icon.clone())
                .large()
                .tooltip("Large icon button")
                .label("Label"),
        )
        .push(
            button::icon(icon.clone())
                .large()
                .on_press(Message::ShowContext(context_header))
                .tooltip("Large icon button")
                .label("Label")
                .vertical(true),
        )
        .push(
            button::icon(icon.clone())
                .large()
                .tooltip("Large icon button")
                .label("Label")
                .vertical(true),
        )
        .insert_row()
        .push(
            button::icon(icon.clone())
                .extra_large()
                .on_press(Message::ShowContext(context_header))
                .tooltip("Extra large icon button"),
        )
        .push(
            button::icon(icon.clone())
                .extra_large()
                .on_press(Message::ShowContext(context_header))
                .tooltip("Extra large icon button")
                .label("Label"),
        )
        .push(
            button::icon(icon.clone())
                .extra_large()
                .tooltip("Extra large icon button")
                .label("Label"),
        )
        .push(
            button::icon(icon.clone())
                .extra_large()
                .on_press(Message::ShowContext(context_header))
                .tooltip("Extra large icon button")
                .label("Label")
                .vertical(true),
        )
        .push(
            button::icon(icon)
                .extra_large()
                .tooltip("Extra large icon button")
                .label("Label")
                .vertical(true),
        )
}
