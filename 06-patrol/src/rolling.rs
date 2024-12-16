use std::collections::VecDeque;

/// RollingArray is a fixed-size buffer with rolling behavior.
/// Generic `T` allows for any data type.
/// `capacity` defines the maximum size of the buffer.
pub struct RollingArray<T> {
    buffer: VecDeque<T>,
    capacity: usize,
}

impl<T> RollingArray<T> {
    /// Creates a new RollingArray with a specified capacity.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than 0");
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Adds an item to the buffer.
    /// If the buffer is full, it removes the oldest item.
    pub fn push(&mut self, item: T) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front(); // Remove the oldest element
        }
        self.buffer.push_back(item); // Add the new element
    }

    /// Returns the current size of the buffer.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Allows access to elements by index.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.buffer.get(index)
    }

    /// Provides an iterator over the elements.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.buffer.iter()
    }
}
