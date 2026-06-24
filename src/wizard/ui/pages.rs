//! UI page builders for the wizard
//!
//! This module contains the implementation for building different page types
//! (welcome, file, directory, password, question, text, warning, error, complete).

use iced::{
    widget::{button, column, container, row, text, text_input, Row},
    Element, Length, alignment, Alignment, Color,
};

use super::super::types::{Message, PageType, WizardWindow, CurrentPage};

impl WizardWindow {
    pub(crate) fn build_content_area(&self) -> Element<'_, Message> {
        if let Some(page) = &self.current_page {
            match page.page_type {
                PageType::Welcome => self.build_welcome_page(page),
                PageType::File => self.build_file_page(page),
                PageType::Directory => self.build_directory_page(page),
                PageType::Password => self.build_password_page(page),
                PageType::Question => self.build_question_page(page),
                PageType::Text => self.build_text_page(page),
                PageType::Warning => self.build_warning_page(page),
                PageType::Error => self.build_error_page(page),
                PageType::Complete => self.build_complete_page(page),
            }
        } else {
            container(text("Ready").size(24))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
                .into()
        }
    }

    fn build_welcome_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        column![
            text(&page.title).size(24),
            text(&page.message).size(14),
            button(text("Next")).on_press(Message::NextClicked),
        ]
        .spacing(12)
        .padding(24)
        .into()
    }

    fn build_file_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        let mut col = column![
            text(&page.title).size(20),
        ];

        if !page.message.is_empty() {
            col = col.push(text(&page.message).size(14));
        }

        col = col.push(
            row![
                text_input(&page.placeholder, &self.file_path)
                    .on_input(Message::FilePathChanged),
                button(text("Browse...")).on_press(Message::BrowseFile),
            ]
            .spacing(6)
        );

        col = col.push(
            row![
                button(text("Cancel")).on_press(Message::CancelClicked),
                button(text("Next")).on_press(Message::NextClicked),
            ]
            .spacing(6)
        );

        col.spacing(12).padding(24).into()
    }

    fn build_directory_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        let mut col = column![
            text(&page.title).size(20),
        ];

        if !page.message.is_empty() {
            col = col.push(text(&page.message).size(14));
        }

        col = col.push(
            row![
                text_input(&page.placeholder, &self.file_path)
                    .on_input(Message::FilePathChanged),
                button(text("Browse...")).on_press(Message::BrowseDirectory),
            ]
            .spacing(6)
        );

        col = col.push(
            row![
                button(text("Cancel")).on_press(Message::CancelClicked),
                button(text("Next")).on_press(Message::NextClicked),
            ]
            .spacing(6)
        );

        col.spacing(12).padding(24).into()
    }

    fn build_password_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        let mut col = column![
            text(&page.title).size(20),
        ];

        if !page.message.is_empty() {
            col = col.push(text(&page.message).size(14));
        }

        col = col.push(text("Password:"));
        col = col.push(
            text_input("", &self.password_input)
                .on_input(Message::PasswordChanged)
                .secure(true)
        );

        if page.confirm {
            col = col.push(text("Confirm Password:"));
            col = col.push(
                text_input("", &self.confirm_password_input)
                    .on_input(Message::ConfirmPasswordChanged)
                    .secure(true)
            );
        }

        if let Some(err) = &self.validation_error {
            col = col.push(text(err).size(12));
        }

        col = col.push(
            row![
                button(text("Cancel")).on_press(Message::CancelClicked),
                button(text("Next")).on_press(Message::NextClicked),
            ]
            .spacing(6)
        );

        col.spacing(12).padding(24).into()
    }

    fn build_question_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        let mut col = column![
            text(&page.title).size(20),
            text(&page.message).size(14),
        ];

        let mut button_row = Row::new().spacing(6);
        for btn_text in &page.buttons {
            button_row = button_row.push(
                button(text(btn_text)).on_press(Message::ButtonClicked(btn_text.clone()))
            );
        }

        col = col.push(button_row);

        col.spacing(12).padding(24).into()
    }

    fn build_text_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        let mut col = column![
            text(&page.title).size(20),
        ];

        if !page.message.is_empty() {
            col = col.push(text(&page.message).size(14));
        }

        col = col.push(
            text_input(&page.placeholder, &self.text_input)
                .on_input(Message::TextChanged)
        );

        if let Some(err) = &self.validation_error {
            col = col.push(text(err).size(12));
        }

        col = col.push(
            row![
                button(text("Cancel")).on_press(Message::CancelClicked),
                button(text("Next")).on_press(Message::NextClicked),
            ]
            .spacing(6)
        );

        col.spacing(12).padding(24).into()
    }

    fn build_warning_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        let warning_icon = container(
            text("!").size(48)
        )
        .padding(16)
        .style(|_theme| {
            container::Style {
                background: Some(Color::from_rgb(0.95, 0.75, 0.0).into()), // Yellow/amber
                text_color: Some(Color::BLACK),
                border: iced::Border {
                    color: Color::from_rgb(0.8, 0.6, 0.0),
                    width: 2.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            }
        });

        column![
            warning_icon,
            text(&page.title).size(20),
            text(&page.message).size(14),
            button(text("OK")).on_press(Message::NextClicked),
        ]
        .spacing(12)
        .padding(24)
        .align_x(Alignment::Center)
        .into()
    }

    fn build_error_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        let error_icon = container(
            text("X").size(48)
        )
        .padding(16)
        .style(|_theme| {
            container::Style {
                background: Some(Color::from_rgb(0.9, 0.2, 0.2).into()), // Red
                text_color: Some(Color::WHITE),
                border: iced::Border {
                    color: Color::from_rgb(0.7, 0.1, 0.1),
                    width: 2.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            }
        });

        column![
            error_icon,
            text(&page.title).size(20),
            text(&page.message).size(14),
            button(text("OK")).on_press(Message::NextClicked),
        ]
        .spacing(12)
        .padding(24)
        .align_x(Alignment::Center)
        .into()
    }

    fn build_complete_page<'a>(&'a self, page: &'a CurrentPage) -> Element<'a, Message> {
        column![
            text("✓").size(72),  // Try checkmark - fallback to "OK" if needed
            text(&page.title).size(20),
            text(&page.message).size(14),
            button(text("Finish")).on_press(Message::FinishClicked),
        ]
        .spacing(12)
        .padding(24)
        .align_x(Alignment::Center)
        .into()
    }
}
