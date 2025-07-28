use crate::selectable::SelectableItem;
use std::ops::Index;

/// A collection of requested items that acts as a wrapper around a Vec<SelectableItem<T>>
pub struct RequestedItems<T> {
    items: Vec<SelectableItem<T>>,
}

impl<T> RequestedItems<T> {
    /// Create a new empty RequestedItems collection
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Create a RequestedItems from a vector of SelectableItem<T>
    pub fn from_vec(items: Vec<SelectableItem<T>>) -> Self {
        Self { items }
    }

    /// Add an item to the collection
    pub fn push(&mut self, item: SelectableItem<T>) {
        self.items.push(item);
    }

    /// Get the number of items in the collection
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get an iterator over the items
    pub fn iter(&self) -> std::slice::Iter<SelectableItem<T>> {
        self.items.iter()
    }

    /// Get a mutable iterator over the items
    pub fn iter_mut(&mut self) -> std::slice::IterMut<SelectableItem<T>> {
        self.items.iter_mut()
    }

    /// Get a reference to the underlying vector
    pub fn as_vec(&self) -> &Vec<SelectableItem<T>> {
        &self.items
    }

    /// Get a mutable reference to the underlying vector
    pub fn as_vec_mut(&mut self) -> &mut Vec<SelectableItem<T>> {
        &mut self.items
    }

    /// Clear all items from the collection
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl<T> Default for RequestedItems<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for RequestedItems<T> {
    type Item = SelectableItem<T>;
    type IntoIter = std::vec::IntoIter<SelectableItem<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a RequestedItems<T> {
    type Item = &'a SelectableItem<T>;
    type IntoIter = std::slice::Iter<'a, SelectableItem<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut RequestedItems<T> {
    type Item = &'a mut SelectableItem<T>;
    type IntoIter = std::slice::IterMut<'a, SelectableItem<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

impl<T> Index<usize> for RequestedItems<T> {
    type Output = SelectableItem<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T> Extend<SelectableItem<T>> for RequestedItems<T> {
    fn extend<I: IntoIterator<Item = SelectableItem<T>>>(&mut self, iter: I) {
        self.items.extend(iter);
    }
}
