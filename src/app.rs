use std::{error, sync::Arc};

use nucleo::{pattern::CaseMatching, Config, Injector, Nucleo, Snapshot};

use crate::selectable::Selectable;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    pub matcher: Nucleo<Selectable<String>>,
    pub injector: Injector<Selectable<String>>,
    // TODO we can probably remove items since we have a matcher now
    // pub items: Vec<String>,
    pub current_index: u32,
    // pub selected: Vec<usize>,
    pub query: String,
    // pub filtered_indices: Vec<usize>,
}

impl App {
    pub fn new() -> Self {
        let matcher = Nucleo::new(Config::DEFAULT, Arc::new(|| {}), None, 1);
        let injector = matcher.injector();
        App {
            matcher,
            injector,
            // items: Vec::new(),
            current_index: 0,
            // selected: Vec::new(),
            query: String::new(),
            // filtered_indices: Vec::new(),
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

    pub fn selected_items(&self) -> Vec<&String> {
        self.snapshot()
            .matched_items(..)
            .filter(|i| i.data.is_selected())
            .map(|i| i.data.value())
            .collect()
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

        // let indices: Vec<usize> = if self.filtered_indices.is_empty() {
        //     (0..self.items.len()).collect::<Vec<_>>()
        // } else {
        //     self.filtered_indices.clone()
        // };

        // if indices.is_empty() {
        //     return;
        // }

        // let current_pos: usize = indices
        //     .iter()
        //     .position(|&i| i == self.current_index)
        //     .unwrap_or(0);
        // let next_pos = (current_pos + 1) % indices.len();
        // self.current_index = indices[next_pos];

        self.current_index = (self.current_index + 1) % indices;
    }

    pub fn previous(&mut self) {
        let indices = self.snapshot().matched_item_count();

        if self.snapshot().matched_item_count() == 0 {
            return;
        }

        // let indices: Vec<usize> = if self.filtered_indices.is_empty() {
        //     (0..self.items.len()).collect::<Vec<_>>()
        // } else {
        //     self.filtered_indices.clone()
        // };

        // if indices.is_empty() {
        //     return;
        // }

        // let current_pos: usize = indices
        //     .iter()
        //     .position(|&i| i == self.current_index)
        //     .unwrap_or(0);
        // let previous_pos = if current_pos == 0 {
        //     indices.len() - 1
        // } else {
        //     current_pos - 1
        // };
        // self.current_index = indices[previous_pos];

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

        // if let Some(pos) = self.selected.iter().position(|&i| i == self.current_index) {
        //     self.selected.remove(pos);
        // } else {
        //     self.selected.push(self.current_index);
        // }
    }

    pub fn update_search(&mut self, query: &str) {
        self.query = query.to_string();

        // if query.is_empty() {
        //     self.filtered_indices.clear();
        //     return;
        // }

        // Use nucleo for matching
        let matcher = nucleo::pattern::Pattern::new(
            query,
            nucleo::pattern::CaseMatching::Smart,
            nucleo::pattern::AtomKind::Exact,
        );

        // self.filtered_indices = self
        //     .items
        //     .iter()
        //     .enumerate()
        //     .filter_map(|(idx, item)| matcher.fuzzy_match(item).map(|_| idx))
        //     .collect();

        // // Reset current index to first match if we have matches
        // if !self.filtered_indices.is_empty() {
        //     self.current_index = self.filtered_indices[0];
        // }
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
}
