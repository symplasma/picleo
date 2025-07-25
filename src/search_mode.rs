use crate::{
    picker::{EventResponse, Picker},
    selectable::SelectableItem,
    selected_items::SelectedItems,
};
use comma::parse_command;
use crossterm::event::{Event, KeyCode, KeyModifiers, MouseButton, MouseEventKind};
use eunicode::{raw_bytes::RawBytes, unicode_string::UnicodeString};
use nucleo::pattern::{CaseMatching, Normalization};
use std::{fmt::Display, ops::RangeInclusive, process::Command};

impl<T> Picker<T>
where
    T: Sync + Send + Display,
{
    /// Handle event processing when we are in search mode
    pub(crate) fn search_mode_handle_event(&mut self, event: Event) -> EventResponse {
        let mut event_response: EventResponse;

        match event {
            Event::Key(key) => {
                event_response = EventResponse::UpdateUI;

                match (key.code, key.modifiers) {
                    (KeyCode::Char(key), KeyModifiers::NONE)
                    | (KeyCode::Char(key), KeyModifiers::SHIFT) => {
                        self.append_to_query(key);
                        // NOTE: this probably doesn't need to saturate, that would require an absurdly long query
                        self.query_index = self.query_index.saturating_add(1);
                    }
                    (KeyCode::Backspace, KeyModifiers::NONE) => {
                        self.delete_from_query();
                        // NOTE: this needs to saturate to handle deleting when the query is empty
                        self.query_index = self.query_index.saturating_sub(1);
                    }
                    // TODO find out if it's a local keybinding that's preventing `Ctrl + Backspace` from working or if it's actually a bug
                    (KeyCode::Backspace, KeyModifiers::CONTROL)
                    | (KeyCode::Backspace, KeyModifiers::ALT) => {
                        self.delete_word_backward();
                    }
                    (KeyCode::Right, KeyModifiers::NONE) => {
                        // NOTE: this probably doesn't need to saturate, that would require an absurdly long query
                        self.query_index = self.query_index.saturating_add(1);
                    }
                    (KeyCode::Right, KeyModifiers::CONTROL)
                    | (KeyCode::Right, KeyModifiers::ALT) => {
                        self.jump_word_forward();
                    }
                    (KeyCode::Left, KeyModifiers::NONE) => {
                        // NOTE: this needs to saturate to handle deleting when the query is empty
                        self.query_index = self.query_index.saturating_sub(1);
                    }
                    (KeyCode::Left, KeyModifiers::CONTROL) | (KeyCode::Left, KeyModifiers::ALT) => {
                        self.jump_word_backward();
                    }
                    (KeyCode::Delete, KeyModifiers::CONTROL)
                    | (KeyCode::Delete, KeyModifiers::ALT) => {
                        self.delete_word_forward();
                    }
                    (KeyCode::Esc, KeyModifiers::NONE) => {
                        if self.query_is_empty() {
                            event_response = EventResponse::ExitProgram;
                        } else {
                            self.clear_query();
                        }
                    }
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        event_response = EventResponse::ExitProgram;
                    }
                    (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                        self.clear_query();
                        self.query_index = 0;
                    }
                    (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                        self.query_index = 0;
                    }
                    (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                        self.query_index = self.query.len();
                    }
                    (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                        self.delete_to_end();
                    }
                    (KeyCode::Enter, KeyModifiers::NONE) => {
                        // Print selected items and exit
                        event_response = EventResponse::ReturnSelectedItems;
                    }
                    (KeyCode::Down, KeyModifiers::NONE) => {
                        self.next();
                    }
                    (KeyCode::PageDown, KeyModifiers::NONE) => {
                        self.next_page();
                    }
                    (KeyCode::End, KeyModifiers::NONE) => {
                        self.end();
                    }
                    (KeyCode::Up, KeyModifiers::NONE) => {
                        self.previous();
                    }
                    (KeyCode::PageUp, KeyModifiers::NONE) => {
                        self.previous_page();
                    }
                    (KeyCode::Home, KeyModifiers::NONE) => {
                        self.home();
                    }
                    (KeyCode::Tab, KeyModifiers::NONE) => {
                        self.toggle_selected();
                        self.next();
                    }
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                        self.enter_editing_mode(self.current_item_text());
                    }
                    (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                        self.enter_editing_mode(String::new());
                    }

                    // ignore other key codes
                    _ => {
                        event_response = EventResponse::NoAction;
                    }
                }
            }
            Event::Mouse(mouse) => {
                event_response = EventResponse::UpdateUI;
                match mouse.kind {
                    MouseEventKind::ScrollUp => {
                        // Check if Shift or Control is held for page navigation
                        if mouse.modifiers.contains(KeyModifiers::SHIFT)
                            || mouse.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            if self.config.invert_scroll() {
                                self.next_page();
                            } else {
                                self.previous_page();
                            }
                        } else {
                            if self.config.invert_scroll() {
                                self.next();
                            } else {
                                self.previous();
                            }
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        // Check if Shift or Control is held for page navigation
                        if mouse.modifiers.contains(KeyModifiers::SHIFT)
                            || mouse.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            if self.config.invert_scroll() {
                                self.previous_page();
                            } else {
                                self.next_page();
                            }
                        } else {
                            if self.config.invert_scroll() {
                                self.previous();
                            } else {
                                self.next();
                            }
                        }
                    }
                    MouseEventKind::Down(MouseButton::Left) => {
                        // Handle left click on item lines to toggle selection
                        self.handle_item_click(mouse.row);
                    }
                    MouseEventKind::Down(MouseButton::Middle) => {
                        self.toggle_selected();
                    }
                    _ => {
                        event_response = EventResponse::NoAction;
                    }
                }
            }
            _ => {
                event_response = EventResponse::NoAction;
            }
        }
        event_response
    }

    pub(crate) fn append_to_query(&mut self, key: char) {
        // TODO constrain selected item to match range
        if self.query_index >= self.query.len() {
            self.query.push(key);
        } else {
            self.query.insert(self.query_index, key);
        }
        self.matcher.pattern.reparse(
            0,
            &self.query,
            CaseMatching::Smart,
            Normalization::Smart,
            true,
        );
        // ensure that the selection stays in range
        // TODO find a better way, ideally one that preserves the position as much as possible
        self.set_current_index(0, Some(false));
        self.update_preview();
    }

    pub(crate) fn delete_from_query(&mut self) {
        if self.query_index > 0 && !self.query.is_empty() {
            // Remove the character before the cursor
            self.query.remove(self.query_index - 1);
        }
        self.matcher.pattern.reparse(
            0,
            &self.query,
            CaseMatching::Smart,
            Normalization::Smart,
            false,
        );
    }

    pub(crate) fn delete_word_backward(&mut self) {
        if self.query_index == 0 {
            return;
        }

        // Get the part of the query before the current position
        let before_cursor = &self.query[..self.query_index];

        // Find the previous word boundary
        let chars: Vec<char> = before_cursor.chars().collect();
        let mut pos = chars.len() - 1;

        // Skip any whitespace before the cursor
        while pos > 0 && chars[pos].is_whitespace() {
            pos -= 1;
        }

        // Skip the current word
        while pos > 0 && !chars[pos].is_whitespace() {
            pos -= 1;
        }

        // If we stopped at whitespace and we're not at the beginning, move to the next char
        if pos > 0 && chars[pos].is_whitespace() {
            pos += 1;
        }

        // Remove the characters between the new position and the old cursor position
        self.query = format!("{}{}", &self.query[..pos], &self.query[self.query_index..]);
        self.query_index = pos;

        // Update the matcher
        self.matcher.pattern.reparse(
            0,
            &self.query,
            CaseMatching::Smart,
            Normalization::Smart,
            false,
        );
    }

    pub(crate) fn delete_word_forward(&mut self) {
        let query_len = self.query.len();
        if self.query_index >= query_len {
            return;
        }

        // Start from current position
        let remaining = &self.query[self.query_index..];

        // Find the next word boundary
        let mut chars = remaining.char_indices();
        let mut end_pos = query_len;

        // If we're at the beginning of a word, delete that word
        if let Some((_, first_char)) = chars.next() {
            if !first_char.is_whitespace() {
                // Skip until we hit whitespace or end
                while let Some((i, c)) = chars.next() {
                    if c.is_whitespace() {
                        end_pos = self.query_index + i;
                        break;
                    }
                }
            } else {
                // Skip whitespace
                while let Some((_i, c)) = chars.next() {
                    if !c.is_whitespace() {
                        // Then skip until next whitespace or end
                        while let Some((j, c2)) = chars.next() {
                            if c2.is_whitespace() {
                                end_pos = self.query_index + j;
                                break;
                            }
                        }
                        break;
                    }
                }
            }
        }

        // Remove the characters between the cursor position and the end position
        self.query = format!(
            "{}{}",
            &self.query[..self.query_index],
            &self.query[end_pos..]
        );

        // Update the matcher
        self.matcher.pattern.reparse(
            0,
            &self.query,
            CaseMatching::Smart,
            Normalization::Smart,
            false,
        );
    }

    pub(crate) fn delete_to_end(&mut self) {
        if self.query_index >= self.query.len() {
            return;
        }

        // Truncate the query at the cursor position
        self.query.truncate(self.query_index);

        // Update the matcher
        self.matcher.pattern.reparse(
            0,
            &self.query,
            CaseMatching::Smart,
            Normalization::Smart,
            false,
        );
    }

    pub(crate) fn jump_word_forward(&mut self) {
        let query_len = self.query.len();
        if self.query_index >= query_len {
            return;
        }

        // Start from current position
        let remaining = &self.query[self.query_index..];

        // Find the next word boundary
        let mut chars = remaining.char_indices();

        // Skip the current word if we're in the middle of one
        while let Some((i, c)) = chars.next() {
            if c.is_whitespace() {
                break;
            }
            if i == remaining.len() - 1 {
                // If we reach the end of the string, set index to the end
                self.query_index = query_len;
                return;
            }
        }

        // Skip any whitespace
        let mut word_start = 0;
        while let Some((i, c)) = chars.next() {
            if !c.is_whitespace() {
                word_start = i;
                break;
            }
            if i == remaining.len() - 1 {
                // If we reach the end of the string, set index to the end
                self.query_index = query_len;
                return;
            }
        }

        // Move to the start of the next word
        self.query_index += word_start;
    }

    pub(crate) fn jump_word_backward(&mut self) {
        if self.query_index == 0 {
            return;
        }

        // Get the part of the query before the current position
        let before_cursor = &self.query[..self.query_index];

        // Find the previous word boundary
        let chars: Vec<char> = before_cursor.chars().collect();
        let mut pos = chars.len() - 1;

        // Skip any whitespace before the cursor
        while pos > 0 && chars[pos].is_whitespace() {
            pos -= 1;
        }

        // Skip the current word
        while pos > 0 && !chars[pos].is_whitespace() {
            pos -= 1;
        }

        // If we stopped at whitespace and we're not at the beginning, move to the next char
        if pos > 0 && chars[pos].is_whitespace() {
            pos += 1;
        }

        self.query_index = pos;
    }

    pub fn query_is_empty(&self) -> bool {
        self.query.is_empty()
    }

    pub(crate) fn clear_query(&mut self) {
        self.query.clear();
        // TODO seems like there should be a better way to clear the query
        self.matcher.pattern.reparse(
            0,
            &self.query,
            CaseMatching::Smart,
            Normalization::Smart,
            false,
        );
    }

    pub fn next(&mut self) {
        let indices = self.last_item_index();
        if indices == 0 {
            return;
        }

        self.set_current_index((self.current_index + 1).into(), None);
        self.update_preview();
    }

    pub(crate) fn next_page(&mut self) {
        let indices = self.last_visible_item_index();
        if indices == 0 {
            return;
        }

        let next_page_index = if self.current_index < self.last_visible_item_index() {
            self.last_visible_item_index()
        } else {
            self.current_index + self.height() as u32
        };
        self.set_current_index(next_page_index.into(), Some(false));
        self.update_preview();
    }

    pub(crate) fn end(&mut self) {
        let indices = self.last_item_index();
        if indices == 0 {
            return;
        }

        self.set_current_index(indices.into(), Some(false));
        self.update_preview();
    }

    pub fn previous(&mut self) {
        let indices = self.last_item_index();
        if indices == 0 {
            return;
        }

        self.set_current_index(self.current_index as i64 - 1, None);
        self.update_preview();
    }

    pub fn previous_page(&mut self) {
        let indices = self.last_item_index();
        if indices == 0 {
            return;
        }

        let previous_page_index = if self.current_index > self.first_visible_item_index() {
            self.first_visible_item_index().into()
        } else {
            self.current_index as i64 - self.height() as i64
        };
        self.set_current_index(previous_page_index, Some(false));
        self.update_preview();
    }

    pub(crate) fn home(&mut self) {
        let indices = self.last_item_index();
        if indices == 0 {
            return;
        }

        self.set_current_index(0, Some(false));
        self.update_preview();
    }

    pub(crate) fn handle_item_click(&mut self, mouse_row: u16) {
        // Calculate which item was clicked based on the mouse row
        // The items list starts at row 4 (help=1, search=3, items start at 4)
        // and has a border, so the first item is at row 5
        if mouse_row < 5 {
            return; // Click was not on an item
        }

        let item_row = mouse_row - 5; // Adjust for UI layout
        let clicked_index = self.first_visible_item_index + item_row as u32;

        // Check if the clicked index is valid
        if clicked_index >= self.matched_item_count() {
            return;
        }

        // Toggle the selection of the clicked item
        let snapshot = self.snapshot();
        if let Some(item) = snapshot.get_matched_item(clicked_index) {
            item.data.toggle_selected();
        }
    }

    pub fn toggle_selected(&mut self) {
        let snapshot = self.snapshot();

        if snapshot.matched_item_count() == 0 {
            return;
        }

        // get the currently selected item and toggle it's selected state
        if let Some(i) = snapshot.get_matched_item(self.current_index) {
            i.data.toggle_selected();
        };
    }

    // this function should constrain the range to valid values and slide the window if necessary
    // NOTE: we're taking an i64 here so we can handle negative values without truncating on the upper end of inputs
    pub fn set_current_index(&mut self, new_index: i64, wrap_around: Option<bool>) -> u32 {
        let wrap_around = wrap_around.unwrap_or_else(|| self.config.wrap_around());
        // ensure that the index is in range
        self.current_index = if new_index < 0 {
            if wrap_around {
                self.last_item_index()
            } else {
                0
            }
        } else if new_index > self.last_item_index().into() {
            if wrap_around {
                0
            } else {
                self.last_item_index()
            }
        } else {
            new_index.try_into().unwrap()
        };
        self.set_item_window(self.current_index.into(), wrap_around);
        self.current_index
    }

    // TODO maybe make new_index into a u32
    pub fn set_item_window(&mut self, new_index: i64, wrap_around: bool) {
        // ensure that the window contains the index
        // TODO handle wrapping
        if new_index < self.first_visible_item_index.into() {
            self.first_visible_item_index = if new_index < 0 {
                if wrap_around {
                    self.last_item_index().saturating_sub(self.height().into())
                } else {
                    0
                }
            } else {
                new_index.try_into().unwrap()
                // self.first_visible_item_index().saturating_sub(1)
            }
            // these are unsigned ints so they shouldn't be able to go below zero
        } else if new_index > self.last_visible_item_index().into() {
            self.first_visible_item_index = if new_index > self.last_item_index().into() {
                if wrap_around {
                    0
                } else {
                    self.last_item_index().saturating_sub(self.height().into())
                }
            } else {
                new_index as u32 - (self.height() as u32)
            }
        }
        // otherwise we don't need to shift the window
    }

    pub fn current_item_text(&self) -> String {
        let snapshot = self.snapshot();

        if snapshot.matched_item_count() == 0 {
            return String::new();
        }

        // Get the currently selected item's text
        if let Some(current_item) = snapshot.get_matched_item(self.current_index) {
            current_item.data.to_string()
        } else {
            String::new()
        }
    }

    pub fn set_preview_command(&mut self, command: String) {
        self.preview_command = Some(command);
    }

    pub fn has_preview(&self) -> bool {
        self.preview_command.is_some()
    }

    pub fn preview_output(&self) -> &str {
        &self.preview_output
    }

    fn substitute_placeholders(&self, command: &str, item_text: &str, escape: bool) -> String {
        let mut result = command.to_string();

        // TODO make this more lazy/efficient
        // Replace {} and {0} with the whole line (escaped)
        let escaped_item_text = if escape {
            shell_escape::escape(item_text.into())
        } else {
            item_text.into()
        };
        result = result.replace("{}", &escaped_item_text);
        result = result.replace("{0}", &escaped_item_text);

        // TODO add support for user specified delimiters
        // Split the item text by whitespace to get columns
        let columns: Vec<&str> = item_text.split_whitespace().collect();

        // TODO make this more efficient, instead of iterating look for the index in the pattern and pull by offset
        // Replace {1}, {2}, etc. with column values (1-indexed, escaped)
        for (i, column) in columns.iter().enumerate() {
            let placeholder = format!("{{{}}}", i + 1);
            let escaped_column = if escape {
                shell_escape::escape((*column).into())
            } else {
                (*column).into()
            };
            result = result.replace(&placeholder, &escaped_column);
        }

        // TODO: Add support for named column placeholders like {column_name}
        // This would require additional metadata about column names

        result
    }

    pub fn update_preview(&mut self) {
        if let Some(ref command) = self.preview_command.clone() {
            let item_text = self.current_item_text();
            if item_text.is_empty() {
                self.preview_output.clear();
            }

            if let Some(command_parts) = parse_command(&command) {
                let mut command_parts_iter = command_parts.iter();
                if let Some(program) = command_parts_iter.next() {
                    // we are substituting args separately to minimize whitespace issues
                    // we could also substitute the whole command while injecting quoted strings and then split
                    let args: Vec<String> = command_parts_iter
                        .map(|arg| self.substitute_placeholders(arg, &item_text, false))
                        .collect();

                    match Command::new(program).args(&args).output() {
                        Ok(output) => {
                            // TODO make this string safe for display
                            //      handle odd bytes
                            //      remove ansi codes except colors
                            //      clean unicode?

                            let mut preview_bytes: Vec<u8> = output.stdout;

                            // handle output on STDERR
                            if !output.stderr.is_empty() {
                                if !preview_bytes.is_empty() {
                                    preview_bytes
                                        .extend_from_slice("\n--- stderr ---\n".as_bytes());
                                }
                                preview_bytes.extend_from_slice(&output.stderr);
                            }

                            // clean ANSI escapes via the `eunicode` crate, but keep colors
                            let raw_bytes =
                                RawBytes::from_bytes(preview_bytes).strip_ansi_escapes(true);
                            // clean sketchy unicode codepoints
                            self.preview_output =
                                UnicodeString::new(raw_bytes).clean().into_string();
                            return;
                        }
                        Err(e) => {
                            self.preview_output = format!("Error executing preview command: {}", e);
                            return;
                        }
                    }
                }
            }

            // if anything above failed, clear the preview output
            self.preview_output.clear();
        }
    }

    pub(crate) fn first_visible_item_index(&self) -> u32 {
        self.first_visible_item_index
    }

    pub(crate) fn last_visible_item_index(&self) -> u32 {
        // TODO probable need to remove the -1 here
        (self.first_visible_item_index + (self.height() as u32))
            // limiting this so we don't get an out of bounds error before loading items or when there are no matches
            .min(self.last_item_index())
    }

    // this should return a valid range that does not exceed the maximum number of items
    pub(crate) fn visible_item_range(&mut self) -> RangeInclusive<u32> {
        // we must use an inclusive range here or we'll be missing items that will cause some weird issues
        self.first_visible_item_index()..=self.last_visible_item_index()
    }

    pub fn matched_items(&mut self) -> Vec<&SelectableItem<T>> {
        // return if the matcher is empty or passing an inclusive range to matched_items will panic
        if self.snapshot().item_count() == 0 {
            return vec![];
        }

        // can't inline this or we'll have ownership issues
        let item_range = self.visible_item_range();

        self.snapshot()
            // is important to restrict this to the visible range or things get really slow with lots of items
            .matched_items(item_range)
            .map(|i| i.data)
            .collect()
    }

    pub(crate) fn selected_items(&self) -> SelectedItems<T> {
        // Get all selected items as references
        let selected_items: Vec<&SelectableItem<T>> = self
            .snapshot()
            .matched_items(..)
            .filter(|i| i.data.is_selected())
            .map(|i| i.data)
            .collect();

        if !selected_items.is_empty() {
            SelectedItems::from_refs(selected_items)
        } else {
            // If no items are selected, return the current item
            let current_item: Vec<&SelectableItem<T>> = self
                .snapshot()
                .matched_items(..)
                .nth(self.current_index as usize)
                .map(|i| vec![i.data])
                .unwrap_or_default();

            SelectedItems::from_refs(current_item)
        }
    }

    /// Returns the index of the last matched item
    // TODO maybe return an Option<u32> here if there are not items. It might improve flow control
    pub fn last_item_index(&self) -> u32 {
        self.snapshot().matched_item_count().saturating_sub(1)
    }
}
