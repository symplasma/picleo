use crate::{
    picker::{EventResponse, Picker},
    selectable::SelectableItem,
};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::{char, fmt::Display};

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
                // TODO try and find a way to actually detect this
                //      though it may not be possible without having users modify their terminal emulator config
                (KeyCode::Backspace, KeyModifiers::CONTROL)
                | (KeyCode::Backspace, KeyModifiers::SHIFT) => {
                    self.delete_word_backward_editing();
                    self.update_autocomplete_suggestions();
                    EventResponse::UpdateUI
                }
                (KeyCode::Delete, KeyModifiers::NONE) => {
                    self.delete_forward_editing();
                    self.update_autocomplete_suggestions();
                    EventResponse::UpdateUI
                }
                (KeyCode::Delete, KeyModifiers::CONTROL)
                | (KeyCode::Delete, KeyModifiers::SHIFT) => {
                    self.delete_word_forward_editing();
                    self.update_autocomplete_suggestions();
                    EventResponse::UpdateUI
                }
                (KeyCode::Right, KeyModifiers::NONE) => {
                    self.editing_index = (self.editing_index + 1).min(self.editing_text.len());
                    EventResponse::UpdateUI
                }
                (KeyCode::Right, KeyModifiers::CONTROL) | (KeyCode::Right, KeyModifiers::SHIFT) => {
                    self.jump_word_forward_editing();
                    EventResponse::UpdateUI
                }
                (KeyCode::Left, KeyModifiers::NONE) => {
                    self.editing_index = self.editing_index.saturating_sub(1);
                    EventResponse::UpdateUI
                }
                (KeyCode::Left, KeyModifiers::CONTROL) | (KeyCode::Left, KeyModifiers::SHIFT) => {
                    self.jump_word_backward_editing();
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
                (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                    self.delete_to_end_of_line_editing();
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

    pub(crate) fn delete_forward_editing(&mut self) {
        if self.editing_index < self.editing_text.len() {
            self.editing_text.remove(self.editing_index);
        }
    }

    pub(crate) fn delete_to_end_of_line_editing(&mut self) {
        self.editing_text.truncate(self.editing_index);
    }

    pub(crate) fn delete_word_backward_editing(&mut self) {
        if self.editing_index == 0 {
            return;
        }

        // Get the part of the text before the current position
        let before_cursor = &self.editing_text[..self.editing_index];

        // Find the previous word boundary
        let chars: Vec<char> = before_cursor.chars().collect();
        let mut new_index = self.editing_index;

        // Skip any whitespace at the current position
        while new_index > 0 && is_skippable_char(&chars[new_index - 1]) {
            new_index -= 1;
        }

        // Skip the current word (non-whitespace characters)
        while new_index > 0 && !is_skippable_char(&chars[new_index - 1]) {
            new_index -= 1;
        }

        // Remove the characters from new_index to current position
        self.editing_text.drain(new_index..self.editing_index);
        self.editing_index = new_index;
    }

    pub(crate) fn delete_word_forward_editing(&mut self) {
        let text_len = self.editing_text.len();
        if self.editing_index >= text_len {
            return;
        }

        // Start from current position
        let remaining = &self.editing_text[self.editing_index..];

        // Find the next word boundary
        let chars: Vec<char> = remaining.chars().collect();
        let mut end_index = 0;

        // Skip any whitespace at the current position
        while end_index < chars.len() && is_skippable_char(&chars[end_index]) {
            end_index += 1;
        }

        // Skip the current word (non-whitespace characters)
        while end_index < chars.len() && !is_skippable_char(&chars[end_index]) {
            end_index += 1;
        }

        // Remove the characters from current position to end_index
        let absolute_end_index = self.editing_index + end_index;
        self.editing_text
            .drain(self.editing_index..absolute_end_index);
    }

    pub(crate) fn jump_word_forward_editing(&mut self) {
        let text_len = self.editing_text.len();
        if self.editing_index >= text_len {
            return;
        }

        // Start from current position
        let remaining = &self.editing_text[self.editing_index..];

        // Find the next word boundary
        let chars: Vec<char> = remaining.chars().collect();
        let mut new_index = 0;

        // Skip any whitespace at the current position
        while new_index < chars.len() && is_skippable_char(&chars[new_index]) {
            new_index += 1;
        }

        // Skip the current word (non-whitespace characters)
        while new_index < chars.len() && !is_skippable_char(&chars[new_index]) {
            new_index += 1;
        }

        self.editing_index = (self.editing_index + new_index).min(text_len);
    }

    pub(crate) fn jump_word_backward_editing(&mut self) {
        if self.editing_index == 0 {
            return;
        }

        // Get the part of the text before the current position
        let before_cursor = &self.editing_text[..self.editing_index];

        // Find the previous word boundary
        let chars: Vec<char> = before_cursor.chars().collect();
        let mut new_index = self.editing_index;

        // Skip any whitespace at the current position
        while new_index > 0 && is_skippable_char(&chars[new_index - 1]) {
            new_index -= 1;
        }

        // Skip the current word (non-whitespace characters)
        while new_index > 0 && !is_skippable_char(&chars[new_index - 1]) {
            new_index -= 1;
        }

        self.editing_index = new_index;
    }
}

fn is_skippable_char(char: &char) -> bool {
    char.is_whitespace() || *char == '/'
}
