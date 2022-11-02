//! # Alda
//!
//! Alda is a crate which implements various data structure and algorithms.
//! It's not no intended to be used in production. It's mainly for educational
//! purpose.

#![forbid(clippy::unwrap_used)]
#![warn(
    rustdoc::missing_doc_code_examples,
    missing_debug_implementations,
    missing_docs,
    clippy::missing_const_for_fn
)]

pub mod search;
pub mod sort;

use std::ops::{Index, IndexMut};

/// The [`Container`] type is a wrapper around the containing data.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Default)]
pub struct Container<T> {
    data: Vec<T>,
}

impl<T> Container<T> {
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
