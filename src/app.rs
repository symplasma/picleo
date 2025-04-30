use std::{error, sync::Arc};

use nucleo::{Config, Injector, Nucleo, Snapshot};

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
    }

    pub fn previous(&mut self) {
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
}
