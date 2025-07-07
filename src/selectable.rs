use std::{
    fmt::Display,
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Debug)]
pub enum SelectableItem<T> {
    Existing {
        value: T,
        selected: AtomicBool,
    },
    Requested {
        value: String,
        selected: AtomicBool,
    },
}

impl<T: Display> Display for SelectableItem<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectableItem::Existing { value, .. } => write!(f, "{}", value),
            SelectableItem::Requested { value, .. } => write!(f, "{}", value),
        }
    }
}

impl<T> SelectableItem<T> {
    // Create a new unselected Existing instance
    pub fn new(value: T) -> Self {
        Self::Existing {
            value,
            selected: false.into(),
        }
    }

    // Create a new selected Existing instance
    pub fn new_selected(value: T) -> Self {
        Self::Existing {
            value,
            selected: true.into(),
        }
    }

    // Create a new unselected Requested instance
    pub fn new_requested(value: String) -> Self {
        Self::Requested {
            value,
            selected: false.into(),
        }
    }

    // Create a new selected Requested instance
    pub fn new_requested_selected(value: String) -> Self {
        Self::Requested {
            value,
            selected: true.into(),
        }
    }

    // Get a reference to the inner value (only for Existing variant)
    pub fn value(&self) -> Option<&T> {
        match self {
            SelectableItem::Existing { value, .. } => Some(value),
            SelectableItem::Requested { .. } => None,
        }
    }

    // Get a mutable reference to the inner value (only for Existing variant)
    pub fn value_mut(&mut self) -> Option<&mut T> {
        match self {
            SelectableItem::Existing { value, .. } => Some(value),
            SelectableItem::Requested { .. } => None,
        }
    }

    // Get a reference to the string value (only for Requested variant)
    pub fn requested_value(&self) -> Option<&String> {
        match self {
            SelectableItem::Existing { .. } => None,
            SelectableItem::Requested { value, .. } => Some(value),
        }
    }

    // Get the selected state
    pub fn is_selected(&self) -> bool {
        match self {
            SelectableItem::Existing { selected, .. } => selected.load(Ordering::SeqCst),
            SelectableItem::Requested { selected, .. } => selected.load(Ordering::SeqCst),
        }
    }

    // Set the selected state
    pub fn set_selected(&self, selected_state: bool) {
        match self {
            SelectableItem::Existing { selected, .. } => selected.store(selected_state, Ordering::SeqCst),
            SelectableItem::Requested { selected, .. } => selected.store(selected_state, Ordering::SeqCst),
        }
    }

    // Toggle the selected state
    pub fn toggle_selected(&self) {
        match self {
            SelectableItem::Existing { selected, .. } => { selected.fetch_xor(true, Ordering::SeqCst); },
            SelectableItem::Requested { selected, .. } => { selected.fetch_xor(true, Ordering::SeqCst); },
        }
    }

    // Consume the wrapper and return the inner value (only for Existing variant)
    pub fn into_inner(self) -> Option<T> {
        match self {
            SelectableItem::Existing { value, .. } => Some(value),
            SelectableItem::Requested { .. } => None,
        }
    }

    // Check if this is an Existing variant
    pub fn is_existing(&self) -> bool {
        matches!(self, SelectableItem::Existing { .. })
    }

    // Check if this is a Requested variant
    pub fn is_requested(&self) -> bool {
        matches!(self, SelectableItem::Requested { .. })
    }
}
