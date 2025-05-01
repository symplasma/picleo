use std::{error, sync::Arc};

use nucleo::{pattern::CaseMatching, Config, Injector, Nucleo, Snapshot};

use crate::selectable::Selectable;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    pub matcher: Nucleo<Selectable<String>>,
    pub injector: Injector<Selectable<String>>,
    pub current_index: u32,
    pub query: String,
}

impl App {
    pub fn new() -> Self {
        let matcher = Nucleo::new(Config::DEFAULT, Arc::new(|| {}), None, 1);
        let injector = matcher.injector();
        App {
            matcher,
            injector,
            current_index: 0,
            query: String::new(),
        }
    }

    pub fn push(&self, str: &str) {
        self.injector.push(Selectable::new(str.into()), |columns| {
            columns[0] = str.into();
        });
    }

    pub fn tick(&mut self, timeout: u64) {
        self.matcher.tick(timeout);
    }

    pub fn snapshot(&self) -> &Snapshot<Selectable<String>> {
        self.matcher.snapshot()
    }

    pub fn items(&self) -> Vec<&Selectable<String>> {
        self.snapshot().matched_items(..).map(|i| i.data).collect()
    }

    pub(crate) fn lines_to_print(&self) -> Vec<String> {
        // NOTE: matched_items is not factored out due to ownership issues
        let selected_items: Vec<String> = self
            .snapshot()
            .matched_items(..)
            .filter(|i| i.data.is_selected())
            .map(|i| i.data.value().to_owned().clone())
            .collect();

        if selected_items.len() > 0 {
            selected_items
        } else {
            self.snapshot()
                .matched_items(..)
                .nth(self.current_index as usize)
                .map(|i| vec![i.data.value().to_owned()])
                .unwrap_or(vec![String::new()])
        }
    }

    pub fn next(&mut self) {
        let indices = self.snapshot().matched_item_count();
        if indices == 0 {
            return;
        }

        self.current_index = (self.current_index + 1) % indices;
    }

    pub fn previous(&mut self) {
        let indices = self.snapshot().matched_item_count();

        if self.snapshot().matched_item_count() == 0 {
            return;
        }

        self.current_index = if self.current_index == 0 {
            indices - 1
        } else {
            self.current_index.saturating_sub(1)
        };
    }

    pub fn toggle_selected(&mut self) {
        let snapshot = self.snapshot();

        if snapshot.matched_item_count() == 0 {
            return;
        }

        // get the currently selected item and toggle it's selected state
        snapshot.get_matched_item(self.current_index).map(|i| {
            i.data.toggle_selected();
        });
    }

    pub(crate) fn append_to_query(&mut self, key: char) {
        // TODO constrain selected item to match range
        self.query.push(key);
        self.matcher
            .pattern
            .reparse(0, &self.query, CaseMatching::Smart, true);
    }

    pub(crate) fn delete_from_query(&mut self) {
        self.query.pop();
        self.matcher
            .pattern
            .reparse(0, &self.query, CaseMatching::Smart, false);
    }

    pub(crate) fn clear_query(&mut self) {
        self.query.clear();
        // TODO seems like there should be a better way to clear the query
        self.matcher
            .pattern
            .reparse(0, &self.query, CaseMatching::Smart, false);
    }
}
