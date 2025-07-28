use crate::{
    picker::{EventResponse, Picker},
    selectable::SelectableItem,
};
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
                    self.update_autocomplete_suggestions();
                    EventResponse::UpdateUI
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => {
                    self.delete_from_editing_text();
                    self.editing_index = self.editing_index.saturating_sub(1);
                    self.update_autocomplete_suggestions();
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
                    self.update_autocomplete_suggestions();
                    EventResponse::UpdateUI
                }
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    self.create_item_from_editing_text();
                    EventResponse::UpdateUI
                }
                (KeyCode::Up, KeyModifiers::NONE) => {
                    if !self.autocomplete_suggestions.is_empty() {
                        self.autocomplete_index = self.autocomplete_index.saturating_sub(1);
                    }
                    EventResponse::UpdateUI
                }
                (KeyCode::Down, KeyModifiers::NONE) => {
                    if !self.autocomplete_suggestions.is_empty() {
                        self.autocomplete_index = (self.autocomplete_index + 1)
                            .min(self.autocomplete_suggestions.len().saturating_sub(1));
                    }
                    EventResponse::UpdateUI
                }
                (KeyCode::Tab, KeyModifiers::NONE) => {
                    if !self.autocomplete_suggestions.is_empty()
                        && self.autocomplete_index < self.autocomplete_suggestions.len()
                    {
                        self.editing_text = self.autocomplete_suggestions[self.autocomplete_index]
                            .to_string()
                            .clone();
                        self.editing_index = self.editing_text.len();
                        self.update_autocomplete_suggestions();
                    }
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

    pub(crate) fn append_to_editing_text(&mut self, key: char) {
        if self.editing_index >= self.editing_text.len() {
            self.editing_text.push(key);
        } else {
            self.editing_text.insert(self.editing_index, key);
        }
    }

    pub(crate) fn delete_from_editing_text(&mut self) {
        if self.editing_index > 0 && !self.editing_text.is_empty() {
            // Remove the character before the cursor
            self.editing_text.remove(self.editing_index - 1);
        }
    }

    pub(crate) fn clear_editing_text(&mut self) {
        self.editing_text.clear();
        self.editing_index = 0;
        self.autocomplete_suggestions.clear();
        self.autocomplete_index = 0;
    }

    pub(crate) fn create_item_from_editing_text(&mut self) {
        if !self.editing_text.is_empty() {
            let new_item = SelectableItem::new_requested_selected(self.editing_text.clone());
            let injector = self.matcher.injector();
            injector.push(new_item, |item, columns| {
                columns[0] = item.to_string().into()
            });
        }
        self.exit_editing_mode();
    }
}
