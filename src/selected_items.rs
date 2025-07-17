use crate::selectable::SelectableItem;

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
