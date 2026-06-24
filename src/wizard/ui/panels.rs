//! UI panel components for the wizard
//!
//! This module contains the implementation for the info and progress panels
//! that appear in the wizard window.

use iced::widget::{column, text, progress_bar, scrollable, Column};

use super::super::types::{Message, WizardWindow};

impl WizardWindow {
    pub(crate) fn build_info_panel(&self) -> Column<'_, Message> {
        column![
            text(&self.info_title).size(18),
            text(&self.info_description).size(14),
            text(&self.info_help).size(12),
        ]
        .spacing(12)
        .padding(12)
    }

    pub(crate) fn build_progress_panel(&self) -> Column<'_, Message> {
        let progress = if self.total_steps > 0 {
            self.current_step as f32 / self.total_steps as f32
        } else {
            0.0
        };

        let mut col = column![
            progress_bar(0.0..=1.0, progress),
            text(&self.status_text).size(12),
        ]
        .spacing(6)
        .padding(12);

        if !self.log_messages.is_empty() {
            let log_text = self.log_messages.join("\n");
            col = col.push(
                scrollable(text(log_text).size(10))
                    .height(150)
            );
        }

        col
    }
}
