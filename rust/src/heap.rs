//! Heap operations.
//!
//! This module implements various heap operations on the [`Container`](crate::Container) type.

use std::marker::PhantomData;
use std::mem;
use std::ops::{Index, IndexMut};

/// Heap type.
#[derive(Debug, Clone)]
pub struct Heap<T, K> {
    buffer: Vec<T>,
    size: usize,
    marker: PhantomData<K>,
}

impl<T, K> Index<usize> for Heap<T, K> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}

impl<T, K> IndexMut<usize> for Heap<T, K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buffer[index]
    }
}

/// Iterator type over the elements in the heap.
#[derive(Debug)]
pub struct Iter<'a, T> {
    inner: &'a [T],
    pos: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.inner.len() {
            self.pos += 1;
            self.inner.get(self.pos - 1)
        } else {
            None
        }
    }
}

/// Max Heap type
struct MaxHeap;

impl<T, K> Heap<T, K>
where
    T: PartialEq + Eq + Ord + PartialOrd + Clone,
{
    /// Creates new heap.
    pub fn new(buffer: Vec<T>) -> Self {
        Self {
            buffer,
            size: 0,
            marker: PhantomData::default(),
        }
    }

    /// Creates an iterator over the values in the heap.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            inner: &self.buffer[..self.size],
            pos: 0,
        }
    }

    /// Returns the index of the parent of the child at the specified index.
    pub const fn parent(&self, index: usize) -> usize {
        index / 2
    }

    /// Returns the index of the left child for the subtree rooted at the specified index.
    pub const fn left_child(&self, index: usize) -> usize {
        index * 2 + 1
    }

    /// Returns the index of the right child for the subtree rooted at the specified index.
    pub const fn right_child(&self, index: usize) -> usize {
        index * 2 + 2
    }
}
impl<T> Heap<T, MaxHeap>
where
    T: PartialEq + Eq + Ord + PartialOrd + Clone,
{
    /// Re-arrange the element at the specified index so that the subtree rooted
    /// at that index satisfied the max heap property.
    pub fn max_heapify(&mut self, index: usize) {
        let left = self.left_child(index);
        let mut largest = index;
        if left < self.size && self[left] > self[index] {
            largest = left;
        }

        let right = self.right_child(index);
        if right < self.size && self[right] > self[largest] {
            largest = right;
        }

        if largest != index {
            self.buffer.swap(index, largest);
            self.max_heapify(largest);
        }
    }

    /// Build max heap
    pub fn build_max_heap(&mut self) {
        self.size = self.buffer.len();
        for i in (0..self.buffer.len() / 2).rev() {
            self.max_heapify(i)
        }
    }

    /// Returns the maximum element in the heap
    pub fn max(&self) -> Option<&T> {
        self.iter().next()
    }

    /// Sort the heap in increasing order.
    pub fn sort(&mut self) {
        self.build_max_heap();
        for i in (1..self.size).rev() {
            self.buffer.swap(0, i);
            self.size -= 1;
            self.max_heapify(0);
        }
    }

    /// Extract the maximum element in the heap.
    pub fn extract_max(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let max = self.buffer.swap_remove(0);
        self.size -= 1;
        self.max_heapify(0);
        Some(max)
    }

    /// Increase the value of the keys at the specified index.
    /// It returns the value of the previous key.
    pub fn increase_key(&mut self, index: usize, key: T) -> Option<T> {
        if key < self[index] || index > self.size {
            return None;
        }

        let mut index = index;
        let prev = mem::replace(&mut self[index], key);
        let mut parent = self.parent(index);
        while index > 0 && self[parent] < self[index] {
            self.buffer.swap(index, parent);
            index = self.parent(index);
            parent = self.parent(index)
        }

        Some(prev)
    }

    /// Delete the element at the specified index from the max heap.
    pub fn delete_max(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }

        if self[index] < self[self.size - 1] {
            self.buffer.swap(index, self.size - 1);
            let old = self.buffer.pop();
            self.size -= 1;
            self.max_heapify(index);
            old
        } else {
            let old = self.buffer[index].clone();
            self.increase_key(index, self[self.size - 1].clone());
            self.buffer.remove(self.size - 1);
            Some(old)
        }
    }
}

impl Heap<i64, MaxHeap> {
    /// Insert new key into the heap.
    pub fn max_insert_key(&mut self, key: i64) {
        let index = self.size;
        self[index] = i64::MIN;
        self.increase_key(index, key);
        self.size += 1;
    }
}

/// Min Heap type.
#[derive(Debug)]
pub struct MinHeap;

impl<T> Heap<T, MinHeap>
where
    T: PartialEq + Eq + Ord + PartialOrd + Clone,
{
    /// Re-arrange the element at the specified index so that the subtree
    /// rooted at the specified index satisfied the min heap property.
    pub fn min_heapify(&mut self, index: usize) {
        let left = self.left_child(index);
        let mut smallest = index;

        if left < self.size && self[left] < self[index] {
            smallest = left;
        }

        let right = self.right_child(index);
        if right < self.size && self[right] < self[smallest] {
            smallest = right;
        }

        if smallest != index {
            self.buffer.swap(index, smallest);
            self.min_heapify(smallest);
        }
    }

    /// Returns the minimum element in the heap
    pub fn min(&self) -> Option<&T> {
        self.iter().next()
    }

    /// Extract the minimum element in the heap.
    pub fn extract_min(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let min = self.buffer.swap_remove(0);
        self.size -= 1;
        self.min_heapify(0);
        Some(min)
    }

    /// Decrease the key at the specified index.
    /// On success, it returns the old value.
    pub fn decrease_key(&mut self, index: usize, key: T) -> Option<T> {
        if index >= self.size || self[index] < key {
            return None;
        }
        let prev = mem::replace(&mut self.buffer[index], key);
        self.min_heapify(index);
        Some(prev)
    }

    /// Delete min key
    pub fn delete_min(&mut self, index: usize) -> Option<T> {
        if self.size <= index {
            return None;
        }

        if self[index] < self[self.size - 1] {
            self.buffer.swap(index, self.size - 1);
            self.size -= 1;
            self.min_heapify(index);
        } else {
            self.decrease_key(index, self[self.size - 1].clone());
            self.size -= 1;
        }
        self.buffer.pop()
    }
}

impl Heap<i64, MinHeap> {
    /// Insert the key into the min heap.
    pub fn min_insert_key(&mut self, key: i64) {
        let index = self.size;
        self[index] = i64::MAX;
        self.decrease_key(index, key);
        self.size += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::Heap;

    #[test]
    fn max_heapify() {
        let mut heap = Heap::new(vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1]);
        heap.size = heap.buffer.len();
        heap.max_heapify(1);
        assert_eq!(heap.buffer, vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1])
    }

    #[test]
    fn build_max_heap() {
        let mut heap = Heap::new(vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1]);
        heap.build_max_heap();
        assert_eq!(heap.size, heap.buffer.len());
        assert_eq!(heap.buffer, vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1])
    }

    #[test]
    fn sort_heap() {
        let mut heap = Heap::new(vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1]);
        heap.sort();
        assert_eq!(heap.buffer, vec![1, 2, 3, 4, 7, 8, 9, 10, 14, 16])
    }

    #[test]
    fn min_heapify() {
        let mut heap = Heap::new(vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1]);
        heap.size = heap.buffer.len();
        heap.min_heapify(0);
        assert_eq!(heap.buffer, vec![4, 7, 10, 14, 1, 9, 3, 2, 8, 16])
    }

    #[test]
    fn extract_max() {
        let mut heap = Heap::new(vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1]);
        heap.build_max_heap();
        let max = heap.extract_max();
        assert_eq!(max, Some(16));
        assert_eq!(heap.buffer, vec![14, 8, 10, 4, 7, 9, 3, 2, 1])
    }

    #[test]
    fn increase_key() {
        let mut heap = Heap::new(vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1]);
        heap.build_max_heap();
        let prev = heap.increase_key(1, 25);
        assert_eq!(prev, Some(14));
        assert_eq!(heap.buffer, vec![25, 16, 10, 8, 7, 9, 3, 2, 4, 1])
    }
}
