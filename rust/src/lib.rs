//! # Alda
//!
//! Alda is a crate which implements various data structure and algorithms.
//! It's not no intended to be used in production. It's mainly for educational
//! purpose.

#![forbid(clippy::unwrap_used)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    clippy::missing_const_for_fn
)]

pub mod bits;
pub mod error;
pub mod heap;
pub mod list;
pub mod maximum_subarray;
pub mod queue;
pub mod search;
pub mod sort;
pub mod stack;

pub use self::error::Error;

use std::ops::{Index, IndexMut};

use heap::{Heap, Value};

/// The [`Container`] type is a wrapper around the containing data.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Default)]
pub struct Container<T> {
    pub(crate) data: Vec<T>,
}

impl<T> Container<T>
where
    T: PartialOrd + Clone,
{
    /// Creates new container instance.
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    /// Returns true the container is empty, otherwise it returns false.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of items in the container.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Swaps two elements in the container.
    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }

    /// Creates a iterator over a container.
    pub fn iter(&self) -> ContainerIterator<T> {
        ContainerIterator {
            items: &self.data,
            pos: 0,
        }
    }

    /// Returns a reference to the inner data.
    pub fn inner(&self) -> &[T] {
        &self.data
    }

    /// Merges two sorted containers.
    ///
    /// This methods creates a new container and merges in two sorted container.
    /// The resulting container element are the elements with the index in the range
    /// of start..end but in sorted order.
    pub(crate) fn merge(&mut self, start: usize, middle: usize, end: usize) {
        let lhs = self.data[start..middle].to_vec();
        let rhs = self.data[middle..end].to_vec();

        let mut i = 0;
        let mut j = 0;
        for k in start..end {
            if i < lhs.len() && j < rhs.len() && lhs[i] <= rhs[j] || i < lhs.len() && j == rhs.len()
            {
                self[k] = lhs[i].clone();
                i += 1;
            } else {
                self[k] = rhs[j].clone();
                j += 1;
            }
        }
    }

    // Partition the container items in the specified bounds.
    pub(crate) fn partition(&mut self, start: usize, end: usize) -> usize {
        let Some(pivot) = self.iter().last().cloned() else {return 0;};
        let mut last_smallest = start;
        for index in start..end - 1 {
            if self[index] <= pivot {
                self.swap(last_smallest, index);
                last_smallest += 1;
            }
        }
        self.swap(last_smallest, end - 1);
        last_smallest
    }
}
impl<T> Index<usize> for Container<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl<T> IndexMut<usize> for Container<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

/// This type is an iterator over a container.
#[derive(Debug)]
pub struct ContainerIterator<'a, T> {
    items: &'a [T],
    pos: usize,
}

impl<'a, T> Iterator for ContainerIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.items.len() {
            self.pos += 1;
            self.items.get(self.pos - 1)
        } else {
            None
        }
    }
}

/// Merge a list of sorted containers.
pub fn merge_all_into(containers: &[&Container<i64>]) -> Container<i64> {
    let cap = containers.iter().map(|c| c.len()).sum();
    let mut buffer = Vec::with_capacity(cap);
    let mut containers: Vec<_> = containers.iter().map(|c| c.iter()).collect();
    for index in 0..containers.len() {
        if let Some(&key) = containers.get_mut(index).and_then(|i| i.next()) {
            buffer.push(Value { key, index });
        }
    }
    let mut heap = Heap::new(buffer);
    heap.build_min_heap();

    let mut merged = Vec::with_capacity(cap);
    while let Some(Value { key, index }) = heap.extract_min() {
        merged.push(key);
        if let Some(&key) = containers.get_mut(index).and_then(|i| i.next()) {
            heap.min_insert_key(Value { key, index });
        }
    }
    Container::new(merged)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sorted_list() {
        let list = &[
            &Container::new(vec![1, 3, 5, 7]),
            &Container::new(vec![-12, -11, -10, -9, 0]),
            &Container::new(vec![2, 4, 6, 8]),
        ];

        let merge = merge_all_into(list);
        assert_eq!(
            merge,
            Container::new(vec![-12, -11, -10, -9, 0, 1, 2, 3, 4, 5, 6, 7, 8])
        )
    }
}
