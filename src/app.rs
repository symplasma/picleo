use std::error;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    pub items: Vec<String>,
    pub current_index: usize,
    pub selected: Vec<usize>,
    pub query: String,
    pub filtered_indices: Vec<usize>,
}

impl App {
    pub fn new() -> Self {
        App {
            items: Vec::new(),
            current_index: 0,
            selected: Vec::new(),
            query: String::new(),
            filtered_indices: Vec::new(),
        }
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let indices = if self.filtered_indices.is_empty() {
            (0..self.items.len()).collect::<Vec<_>>()
        } else {
            self.filtered_indices.clone()
        };

        if indices.is_empty() {
            return;
        }

        let current_pos = indices
            .iter()
            .position(|&i| i == self.current_index)
            .unwrap_or(0);
        let next_pos = (current_pos + 1) % indices.len();
        self.current_index = indices[next_pos];
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let indices = if self.filtered_indices.is_empty() {
            (0..self.items.len()).collect::<Vec<_>>()
        } else {
            self.filtered_indices.clone()
        };

        if indices.is_empty() {
            return;
        }

        let current_pos = indices
            .iter()
            .position(|&i| i == self.current_index)
            .unwrap_or(0);
        let previous_pos = if current_pos == 0 {
            indices.len() - 1
        } else {
            current_pos - 1
        };
        self.current_index = indices[previous_pos];
    }

    pub fn toggle_selected(&mut self) {
        if self.items.is_empty() {
            return;
        }

        if let Some(pos) = self.selected.iter().position(|&i| i == self.current_index) {
            self.selected.remove(pos);
        } else {
            self.selected.push(self.current_index);
        }
    }

    pub fn update_search(&mut self, query: &str) {
        self.query = query.to_string();

        if query.is_empty() {
            self.filtered_indices.clear();
            return;
        }

        // Use nucleo for matching
        let matcher = nucleo::pattern::Pattern::new(
            query,
            nucleo::pattern::CaseMatching::Smart,
            nucleo::pattern::AtomKind::Exact,
        );

        self.filtered_indices = self
            .items
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| matcher.fuzzy_match(item).map(|_| idx))
            .collect();

        // Reset current index to first match if we have matches
        if !self.filtered_indices.is_empty() {
            self.current_index = self.filtered_indices[0];
        }
    }
}
