use std::{
    fmt::Display,
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Debug)]
pub struct SelectableItem<T> {
    value: T,
    selected: AtomicBool,
}

impl<T: Display> Display for SelectableItem<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> SelectableItem<T> {
    // Create a new unselected instance
    pub fn new(value: T) -> Self {
        Self {
            value,
            selected: false.into(),
        }
    }

    // Create a new selected instance
    pub fn new_selected(value: T) -> Self {
        Self {
            value,
            selected: true.into(),
        }
    }

    // Get a reference to the inner value
    pub fn value(&self) -> &T {
        &self.value
    }

    // Get a mutable reference to the inner value
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    // Get the selected state
    pub fn is_selected(&self) -> bool {
        self.selected.load(Ordering::SeqCst)
    }

    // Set the selected state
    pub fn set_selected(&self, selected: bool) {
        self.selected.store(selected, Ordering::SeqCst);
    }

    // Toggle the selected state
    pub fn toggle_selected(&self) {
        self.selected.fetch_xor(true, Ordering::SeqCst);
    }

    // Consume the wrapper and return the inner value
    pub fn into_inner(self) -> T {
        self.value
    }
}
