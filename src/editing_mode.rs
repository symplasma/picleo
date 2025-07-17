use crate::picker::{EventResponse, Picker};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::fmt::Display;

impl<T> Picker<T>
where
    T: Sync + Send + Display,
{
    /// Handle key events when in editing mode
    pub(crate) fn editing_mode_handle_event(&mut self, event: Event) -> EventResponse {
        match event {
            Event::Key(key) => match (key.code, key.modifiers) {
                (KeyCode::Char(ch), KeyModifiers::NONE)
                | (KeyCode::Char(ch), KeyModifiers::SHIFT) => {
                    self.append_to_editing_text(ch);
                    self.editing_index = self.editing_index.saturating_add(1);
                    EventResponse::UpdateUI
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => {
                    self.delete_from_editing_text();
                    self.editing_index = self.editing_index.saturating_sub(1);
                    EventResponse::UpdateUI
                }
                (KeyCode::Right, KeyModifiers::NONE) => {
                    self.editing_index = (self.editing_index + 1).min(self.editing_text.len());
                    EventResponse::UpdateUI
                }
                (KeyCode::Left, KeyModifiers::NONE) => {
                    self.editing_index = self.editing_index.saturating_sub(1);
                    EventResponse::UpdateUI
                }
                (KeyCode::Home, KeyModifiers::NONE)
                | (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                    self.editing_index = 0;
                    EventResponse::UpdateUI
                }
                (KeyCode::End, KeyModifiers::NONE)
                | (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                    self.editing_index = self.editing_text.len();
                    EventResponse::UpdateUI
                }
                (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                    self.clear_editing_text();
                    EventResponse::UpdateUI
                }
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    self.create_item_from_editing_text();
                    EventResponse::UpdateUI
                }
                (KeyCode::Esc, KeyModifiers::NONE) => {
                    self.exit_editing_mode();
                    EventResponse::UpdateUI
                }
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => EventResponse::ExitProgram,
                _ => EventResponse::NoAction,
            },

            // ignore other event types
            _ => EventResponse::NoAction,
        }
    }
}
