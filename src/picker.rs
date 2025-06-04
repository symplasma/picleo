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
use std::{error, fmt::Display, io, sync::Arc};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

// TODO convert static to a proper lifetime
pub struct Picker<T>
where
    T: Sync + Send + 'static,
{
    pub matcher: Nucleo<SelectableItem<T>>,
    pub current_index: u32,
    pub height: u16,
    pub query: String,
    pub query_index: usize,
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
            current_index: 0,
            height: 0,
            query: String::new(),
            query_index: 0,
        }
    }

    pub fn inject_items<F>(&self, f: F)
    where
        F: FnOnce(&Injector<SelectableItem<T>>),
    {
        let injector = self.matcher.injector();
        f(&injector);
    }

    pub fn tick(&mut self, timeout: u64) {
        self.matcher.tick(timeout);
    }

    pub fn snapshot(&self) -> &Snapshot<SelectableItem<T>> {
        self.matcher.snapshot()
    }

    pub fn items(&self) -> Vec<&SelectableItem<T>> {
        self.snapshot().matched_items(..).map(|i| i.data).collect()
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
        self.query.push(key);
        self.matcher.pattern.reparse(
            0,
            &self.query,
            CaseMatching::Smart,
            Normalization::Smart,
            true,
        );
    }

    pub(crate) fn delete_from_query(&mut self) {
        self.query.pop();
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
        loop {
            self.tick(10);
            terminal.draw(|f| ui(f, self))?;

            if let Ok(Event::Key(key)) = event::read() {
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
                    (KeyCode::Right, KeyModifiers::NONE) => {
                        // NOTE: this probably doesn't need to saturate, that would require an absurdly long query
                        self.query_index = self.query_index.saturating_add(1);
                    }
                    (KeyCode::Left, KeyModifiers::NONE) => {
                        // NOTE: this needs to saturate to handle deleting when the query is empty
                        self.query_index = self.query_index.saturating_sub(1);
                    }
                    // TODO add more editing functions e.g. forward and back, forward delete, word forward/back
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
                    _ => {}
                }
            };
        }
    }
}
