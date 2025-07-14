use crate::{config::Config, selectable::SelectableItem, ui::ui};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseButton,
        MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use nucleo::{
    pattern::{CaseMatching, Normalization},
    Config as NucleoConfig, Injector, Nucleo, Snapshot,
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use shell_escape;
use std::{
    error, fmt::Display, io, ops::RangeInclusive, process::Command, sync::Arc, thread::JoinHandle,
    time::Duration,
};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct SelectedItems<'a, T> {
    items: Vec<&'a SelectableItem<T>>,
}

impl<'a, T> SelectedItems<'a, T> {
    pub fn from_refs(items: Vec<&'a SelectableItem<T>>) -> Self {
        Self { items }
    }

    /// Returns a Vec of references to the inner values from Existing selected items
    pub fn existing_values(&self) -> Vec<&T> {
        self.items.iter().filter_map(|item| item.value()).collect()
    }

    /// Returns a Vec of string references from Requested selected items
    pub fn requested_values(&self) -> Vec<&str> {
        self.items
            .iter()
            .filter_map(|item| item.requested_value().map(|s| s.as_str()))
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PickerMode {
    Search,
    Editing,
}

enum EventResponse {
    NoAction,
    UpdateUI,
    ExitProgram,
    ReturnSelectedItems,
}

// TODO convert static to a proper lifetime
pub struct Picker<T>
where
    T: Sync + Send + 'static,
{
    pub matcher: Nucleo<SelectableItem<T>>,
    pub first_visible_item_index: u32,
    pub current_index: u32,
    pub height: u16,
    pub query: String,
    pub query_index: usize,
    pub mode: PickerMode,
    pub editing_text: String,
    pub editing_index: usize,
    pub join_handles: Vec<JoinHandle<()>>,
    pub config: Config,
    pub preview_command: Option<String>,
    pub preview_output: String,
}

impl<T: Sync + Send + Display> Default for Picker<T> {
    fn default() -> Self {
        Self::new()
    }
}

// TODO maybe expose the Nucleo update callback
impl<T> Picker<T>
where
    T: Sync + Send + Display,
{
    pub fn new() -> Self {
        let config = Config::load().unwrap_or_default();
        let matcher = Nucleo::new(NucleoConfig::DEFAULT, Arc::new(|| {}), None, 1);
        let preview_command = config.preview_command().cloned();
        Picker {
            matcher,
            first_visible_item_index: 0,
            current_index: 0,
            height: config.height().unwrap_or(0),
            query: String::new(),
            query_index: 0,
            mode: PickerMode::Search,
            editing_text: String::new(),
            editing_index: 0,
            join_handles: Vec::new(),
            config,
            preview_command,
            preview_output: String::new(),
        }
    }

    pub fn inject_items<F>(&self, f: F)
    where
        F: FnOnce(&Injector<SelectableItem<T>>),
    {
        let injector = self.matcher.injector();
        f(&injector);
    }

    pub fn inject_items_threaded<F>(&mut self, f: F)
    where
        F: FnOnce(&Injector<SelectableItem<T>>) + Send + 'static,
    {
        let injector = self.matcher.injector();
        let handle = std::thread::spawn(move || {
            f(&injector);
        });
        self.join_handles.push(handle);
    }

    pub fn join_finished_threads(&mut self) -> usize {
        let mut remaining_handles = Vec::new();

        for handle in self.join_handles.drain(..) {
            if handle.is_finished() {
                // Thread is finished, join it (ignore any errors)
                let _ = handle.join();
            } else {
                // Thread is still running, keep it
                remaining_handles.push(handle);
            }
        }

        self.join_handles = remaining_handles;
        self.join_handles.len()
    }

    pub fn running_threads(&self) -> usize {
        self.join_handles.len()
    }

    pub fn item_count(&self) -> u32 {
        self.matcher.snapshot().item_count()
    }

    pub fn height(&self) -> u16 {
        // truncation should be fine since we are getting the min and we don't want this to panic
        self.height.min(self.item_count() as u16)
    }

    pub fn tick(&mut self, timeout: u64) -> nucleo::Status {
        // TODO ensure that this is the correct place to call the thread join
        let _running_indexers = self.join_finished_threads();
        self.matcher.tick(timeout)
    }

    pub fn snapshot(&self) -> &Snapshot<SelectableItem<T>> {
        self.matcher.snapshot()
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

    pub(crate) fn update_height(&mut self, height: u16) {
        self.height = height;
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

    /// Returns the total number of matched items
    pub fn matched_item_count(&self) -> u32 {
        self.snapshot().matched_item_count()
    }

    /// Returns the index of the last matched item
    // TODO maybe return an Option<u32> here if there are not items. It might improve flow control
    pub fn last_item_index(&self) -> u32 {
        self.snapshot().matched_item_count().saturating_sub(1)
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

    pub fn next(&mut self) {
        let indices = self.last_item_index();
        if indices == 0 {
            return;
        }

        self.set_current_index((self.current_index + 1).into(), None);
        self.update_preview();
    }

    fn next_page(&mut self) {
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

    fn end(&mut self) {
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

    fn home(&mut self) {
        let indices = self.last_item_index();
        if indices == 0 {
            return;
        }

        self.set_current_index(0, Some(false));
        self.update_preview();
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

    fn substitute_placeholders(&self, command: &str, item_text: &str) -> String {
        let mut result = command.to_string();

        // Replace {} and {0} with the whole line (escaped)
        let escaped_item_text = shell_escape::escape(item_text.into());
        result = result.replace("{}", &escaped_item_text);
        result = result.replace("{0}", &escaped_item_text);

        // Split the item text by whitespace to get columns
        let columns: Vec<&str> = item_text.split_whitespace().collect();

        // Replace {1}, {2}, etc. with column values (1-indexed, escaped)
        for (i, column) in columns.iter().enumerate() {
            let placeholder = format!("{{{}}}", i + 1);
            let escaped_column = shell_escape::escape((*column).into());
            result = result.replace(&placeholder, &escaped_column);
        }

        // TODO: Add support for named column placeholders like {column_name}
        // This would require additional metadata about column names

        result
    }

    pub fn update_preview(&mut self) {
        if let Some(ref command) = self.preview_command.clone() {
            let item_text = self.current_item_text();
            if !item_text.is_empty() {
                let substituted_command = self.substitute_placeholders(command, &item_text);

                // Parse and execute the command directly
                let mut parts = substituted_command.split_whitespace();
                let program = parts.next().unwrap_or("");
                let args: Vec<&str> = parts.collect();

                match Command::new(program).args(&args).output() {
                    Ok(output) => {
                        self.preview_output = String::from_utf8_lossy(&output.stdout).to_string();
                        if !output.stderr.is_empty() {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            if !self.preview_output.is_empty() {
                                self.preview_output.push_str("\n--- stderr ---\n");
                            }
                            self.preview_output.push_str(&stderr);
                        }
                    }
                    Err(e) => {
                        self.preview_output = format!("Error executing preview command: {}", e);
                    }
                }
            } else {
                self.preview_output.clear();
            }
        }
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

    pub(crate) fn append_to_editing_text(&mut self, key: char) {
        if self.editing_index >= self.editing_text.len() {
            self.editing_text.push(key);
        } else {
            self.editing_text.insert(self.editing_index, key);
        }
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

    pub(crate) fn delete_from_editing_text(&mut self) {
        if self.editing_index > 0 && !self.editing_text.is_empty() {
            // Remove the character before the cursor
            self.editing_text.remove(self.editing_index - 1);
        }
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

    pub(crate) fn clear_editing_text(&mut self) {
        self.editing_text.clear();
        self.editing_index = 0;
    }

    pub(crate) fn enter_editing_mode(&mut self, item_text: String) {
        self.mode = PickerMode::Editing;
        self.editing_text = item_text;
        self.editing_index = 0;
    }

    pub(crate) fn exit_editing_mode(&mut self) {
        self.mode = PickerMode::Search;
        self.editing_text.clear();
        self.editing_index = 0;
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

    pub fn run(&mut self) -> AppResult<SelectedItems<T>> {
        // Setup terminal
        enable_raw_mode()?;
        // TODO should we allow the caller to pass any arbitrary stream?
        let mut stream = io::stderr();
        execute!(stream, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stream);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_loop(&mut terminal);

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }

    pub(crate) fn run_loop<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> AppResult<SelectedItems<T>> {
        // Update preview initially if we have a preview command
        self.update_preview();

        // draw the UI once initially before any timeouts so it appears to the user immediately
        terminal.draw(|f| ui(f, self))?;

        // enter the actual event loop
        loop {
            // we must call this to keep Nucleo up to date
            let status = self.tick(10);
            let mut redraw_requested = false;

            // ensure that we update the UI, even when we aren't receiving events from the user
            if event::poll(Duration::from_millis(16))? {
                // read the event that is ready (normally read blocks, but we're polling until it's ready)
                let event = event::read()?;
                match self.search_mode_handle_event(event) {
                    EventResponse::NoAction => {}
                    EventResponse::UpdateUI => redraw_requested = true,
                    EventResponse::ExitProgram => return Ok(SelectedItems::from_refs(vec![])),
                    EventResponse::ReturnSelectedItems => return Ok(self.selected_items()),
                }
            }

            // redraw the UI if any of the below are true
            //   1. a redraw is requested by an event
            //   2. the matcher's status has changed
            //   3. injectors are still running and adding items
            if redraw_requested || status.changed || status.running {
                // TODO need to debounce events here
                terminal.draw(|f| ui(f, self))?;
            }
        }
    }

    /// Handle event processing when we are in search mode
    fn search_mode_handle_event(&mut self, event: Event) -> EventResponse {
        let mut event_response: EventResponse;

        match event {
            Event::Key(key) => {
                event_response = EventResponse::UpdateUI;

                // Handle mode-specific events first
                if self.mode == PickerMode::Editing {
                    return self.handle_editing_mode_key_event(key);
                }

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

    /// Handle key events when in editing mode
    fn handle_editing_mode_key_event(&mut self, key: crossterm::event::KeyEvent) -> EventResponse {
        match (key.code, key.modifiers) {
            (KeyCode::Char(ch), KeyModifiers::NONE) | (KeyCode::Char(ch), KeyModifiers::SHIFT) => {
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
            (KeyCode::Home, KeyModifiers::NONE) | (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                self.editing_index = 0;
                EventResponse::UpdateUI
            }
            (KeyCode::End, KeyModifiers::NONE) | (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
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
        }
    }
}
