use crate::{selectable::SelectableItem, ui::ui};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use nucleo::{
    pattern::{CaseMatching, Normalization},
    Config, Injector, Nucleo, Snapshot,
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::{
    error, fmt::Display, io, iter::once, ops::Range, sync::Arc, thread::JoinHandle, time::Duration,
};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

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
    pub join_handles: Vec<JoinHandle<()>>,
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
        let matcher = Nucleo::new(Config::DEFAULT, Arc::new(|| {}), None, 1);
        Picker {
            matcher,
            first_visible_item_index: 0,
            current_index: 0,
            height: 0,
            query: String::new(),
            query_index: 0,
            join_handles: Vec::new(),
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
        (self.first_visible_item_index + self.height as u32)
            // limiting this so we don't get an out of bounds error before loading items or when there are no matches
            .min(self.snapshot().matched_item_count())
    }

    pub(crate) fn visible_item_range(&mut self) -> Range<u32> {
        let current_index = self.current_index;

        match (self.first_visible_item_index()..self.last_visible_item_index())
            .cmp(once(current_index))
        {
            std::cmp::Ordering::Less => {
                self.first_visible_item_index = self.first_visible_item_index()
                    + (current_index.saturating_sub(self.last_visible_item_index()))
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => self.first_visible_item_index = self.current_index,
        };

        self.first_visible_item_index()..self.last_visible_item_index()
    }

    pub fn matched_items(&mut self) -> Vec<&SelectableItem<T>> {
        let visible_item_range = self.visible_item_range();
        self.snapshot()
            // is important to restrict this to the visible range or things get really slow with lots of items
            .matched_items(visible_item_range)
            .map(|i| i.data)
            .collect()
    }

    pub(crate) fn update_height(&mut self, height: u16) {
        self.height = height;
    }

    pub(crate) fn selected_items(&self) -> Vec<&T> {
        // NOTE: matched_items is not factored out due to ownership issues
        let selected_items: Vec<&T> = self
            .snapshot()
            .matched_items(..)
            .filter(|i| i.data.is_selected())
            .map(|i| i.data.value())
            .collect();

        if !selected_items.is_empty() {
            selected_items
        } else {
            self.snapshot()
                .matched_items(..)
                .nth(self.current_index as usize)
                .map(|i| vec![i.data.value()])
                .unwrap_or_default()
        }
    }

    pub fn next(&mut self) {
        let indices = self.snapshot().matched_item_count();
        if indices == 0 {
            return;
        }

        self.current_index = (self.current_index + 1) % indices;
    }

    fn next_page(&mut self) {
        let indices = self.snapshot().matched_item_count();
        if indices == 0 {
            return;
        }

        let next_page_index = self.current_index + self.height as u32;
        self.current_index = if next_page_index > indices {
            indices
        } else {
            next_page_index
        }
    }

    fn end(&mut self) {
        let indices = self.snapshot().matched_item_count();
        if indices == 0 {
            return;
        }

        self.current_index = indices;
    }

    pub fn previous(&mut self) {
        let indices = self.snapshot().matched_item_count();
        if indices == 0 {
            return;
        }

        self.current_index = if self.current_index == 0 {
            indices - 1
        } else {
            self.current_index.saturating_sub(1)
        };
    }

    pub fn previous_page(&mut self) {
        let indices = self.snapshot().matched_item_count();
        if indices == 0 {
            return;
        }

        self.current_index = self.current_index.saturating_sub(self.height as u32);
    }

    fn home(&mut self) {
        let indices = self.snapshot().matched_item_count();
        if indices == 0 {
            return;
        }

        self.current_index = 0;
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

    pub fn run(&mut self) -> AppResult<Vec<&T>> {
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
    ) -> AppResult<Vec<&T>> {
        // draw the UI once initially before any timeouts so it appears to the user immediately
        terminal.draw(|f| ui(f, self))?;

        let mut event_received = false;

        // enter the actual event loop
        loop {
            let status = self.tick(10);

            // ensure that we update the UI, even when we aren't receiving events from the user
            if event::poll(Duration::from_millis(16))? {
                // read the event that is ready (normally read blocks, but we're polling until it's ready)
                if let Ok(Event::Key(key)) = event::read() {
                    event_received = true;
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
                        (KeyCode::Left, KeyModifiers::CONTROL)
                        | (KeyCode::Left, KeyModifiers::ALT) => {
                            self.jump_word_backward();
                        }
                        (KeyCode::Delete, KeyModifiers::CONTROL)
                        | (KeyCode::Delete, KeyModifiers::ALT) => {
                            self.delete_word_forward();
                        }
                        (KeyCode::Esc, KeyModifiers::NONE) => {
                            return Ok(vec![]);
                        }
                        (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                            self.clear_query();
                            self.query_index = 0;
                        }
                        (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                            return Ok(vec![]);
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
                            return Ok(self.selected_items());
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

                        // ignore other key codes
                        _ => {
                            event_received = false;
                        }
                    }
                };
            }

            // if necessary, redraw the screen
            if event_received || status.changed || status.running {
                // TODO need to debounce events here
                terminal.draw(|f| ui(f, self))?;
            }
        }
    }
}
