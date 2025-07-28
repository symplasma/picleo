use crate::requested_items::RequestedItems;
use crate::{config::Config, selectable::SelectableItem, selected_items::SelectedItems, ui::ui};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use nucleo::{Config as NucleoConfig, Injector, Nucleo, Snapshot};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::{error, fmt::Display, io, sync::Arc, thread::JoinHandle, time::Duration};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PickerMode {
    Search,
    Editing,
}

pub(crate) enum EventResponse {
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
    pub keep_colors: bool,
    pub editable: bool,
    pub autocomplete: Option<Box<dyn Fn(&str) -> RequestedItems<String> + Send + Sync>>,
    pub autocomplete_suggestions: RequestedItems<String>,
    pub autocomplete_index: usize,
}

impl<T: Sync + Send + Display> Default for Picker<T> {
    fn default() -> Self {
        Self::new(true)
    }
}

// TODO maybe expose the Nucleo update callback
impl<T> Picker<T>
where
    T: Sync + Send + Display,
{
    pub fn new(editable: bool) -> Self {
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
            keep_colors: false,
            editable,
            autocomplete: None,
            autocomplete_suggestions: RequestedItems::default(),
            autocomplete_index: 0,
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

    /// Returns the total number of matched items
    pub fn matched_item_count(&self) -> u32 {
        self.snapshot().matched_item_count()
    }

    pub fn height(&self) -> u16 {
        // truncation should be fine since we are getting the min and we don't want this to panic
        self.height.min(self.item_count() as u16)
    }

    pub(crate) fn update_height(&mut self, height: u16) {
        self.height = height;
    }

    pub fn tick(&mut self, timeout: u64) -> nucleo::Status {
        // TODO ensure that this is the correct place to call the thread join
        let _running_indexers = self.join_finished_threads();
        self.matcher.tick(timeout)
    }

    pub fn snapshot(&self) -> &Snapshot<SelectableItem<T>> {
        self.matcher.snapshot()
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
        self.autocomplete_suggestions.clear();
        self.autocomplete_index = 0;
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
        // setting this to true initially to trigger the initial screen paint
        let mut redraw_requested = true;

        // enter the actual event loop
        loop {
            // draw the UI before any timeouts so it appears to the user immediately
            // redraw the UI if any of the below are true
            //   1. a redraw is requested by an event
            //   2. the matcher's status has changed
            //   3. injectors are still running and adding items
            if redraw_requested {
                terminal.draw(|f| ui(f, self))?;
            }

            // toggling this back to the default, it will be switched back to true below on appropriate conditions
            redraw_requested = false;

            // we must call this to keep Nucleo up to date
            let status = self.tick(10);
            // NOTE: do NOT try to move this logic into the event logic, there are non-event changes that need to trigger redraws
            if status.changed || status.running {
                // TODO need to debounce events here

                // Update preview initially if we have a preview command
                // TODO determine if this is the right place to update the preview
                self.update_preview();

                redraw_requested = true;
            }

            // ensure that we update the UI, even when we aren't receiving events from the user
            if event::poll(Duration::from_millis(16))? {
                // read the event that is ready (normally read blocks, but we're polling until it's ready)
                let event = event::read()?;
                match self.handle_event_by_mode(event) {
                    EventResponse::NoAction => {}
                    EventResponse::UpdateUI => redraw_requested = true,
                    EventResponse::ExitProgram => return Ok(SelectedItems::from_refs(vec![])),
                    EventResponse::ReturnSelectedItems => return Ok(self.selected_items()),
                }
            }
        }
    }

    fn handle_event_by_mode(&mut self, event: Event) -> EventResponse {
        match self.mode {
            PickerMode::Search => self.search_mode_handle_event(event),
            PickerMode::Editing => self.editing_mode_handle_event(event),
        }
    }

    pub fn set_autocomplete<F>(&mut self, autocomplete: F)
    where
        F: Fn(&str) -> RequestedItems<String> + Send + Sync + 'static,
    {
        self.autocomplete = Some(Box::new(autocomplete));
    }

    pub(crate) fn update_autocomplete_suggestions(&mut self) {
        if let Some(ref autocomplete_fn) = self.autocomplete {
            self.autocomplete_suggestions = autocomplete_fn(&self.editing_text);
            self.autocomplete_index = 0;
        }
    }
}
