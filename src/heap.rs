//! Heap data structure routine
//!
//! This module implement the [`Heap`] data structure and the
//! associated methods or functions.
//!

use std::cell::RefCell;
use std::cmp::{Ord, Ordering};
use std::fmt;

/// Heap is a binary heap data structure.
pub struct Heap<T, P>
where
    T: Ord,
    P: Fn(&Node<T>, &Node<T>) -> bool,
{
    pub size: usize,
    predicate: P,
    pub is_sorted: bool,
    nodes: RefCell<Vec<Node<T>>>,
}

impl<T, P> fmt::Debug for Heap<T, P>
where
    T: Ord + fmt::Debug,
    P: Fn(&Node<T>, &Node<T>) -> bool,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Heap").field("nodes", &self.nodes).finish()
    }
}

impl<T, P> Heap<T, P>
where
    T: Ord,
    P: Fn(&Node<T>, &Node<T>) -> bool,
{
    /// Create a new `Heap`.

    ///
    /// # Examples
    ///
    /// ```
    /// use alda::heap::{Heap, Node};
    ///
    /// let h: Heap<_,_> = Heap::new(|x: &Node<isize>, y: &Node<isize>| x < y);
    /// ```
    ///
    pub fn new(predicate: P) -> Heap<T, P> {
        Heap {
            size: 0,
            predicate,
            is_sorted: true,
            nodes: RefCell::new(Vec::<Node<T>>::new()),
        }
    }

    /// Create a new  [`Heap`] from a vector of nodes.
    ///
    /// # Panics
    ///
    /// This method will panic if vector is an empty vector. In which case
    /// you must use [`Heap::new`]
    ///
    /// # Examples
    ///
    /// ```
    /// use alda::heap::{Heap, Node};
    ///
    /// let h = Heap::from(vec![Node::new(1)], |x, y| x > y);
    ///
    pub fn from(nodes: Vec<Node<T>>, predicate: P) -> Heap<T, P> {
        assert!(nodes.len() > 0);

        let mut h = Heap {
            size: nodes.len(),
            predicate,
            is_sorted: true,
            nodes: RefCell::new(nodes),
        };
        h.build();
        h
    }

    /// Maintain the heap property.
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use alda::heap::Heap;
    ///
    /// let mut h: Heap<i32, _> = Heap::new(|x, y| x < y);
    /// h.build();
    /// ```
    ///
    pub fn build(&mut self) {
        if self.size < 2 {
            return;
        }
        for i in (0..self.size / 2).rev() {
            self.heapify(i);
        }
    }

    fn heapify(&self, index: usize) {
        let left = 2 * index + 1;
        let mut largest = index;

        if left < self.size
            && (self.predicate)(&self.nodes.borrow()[left], &self.nodes.borrow()[index])
        {
            largest = left;
        }

        let right = 2 * index + 2;
        if right < self.size
            && (self.predicate)(&self.nodes.borrow()[right], &self.nodes.borrow()[largest])
        {
            largest = right;
        }

        if largest != index {
            self.nodes.borrow_mut().swap(index, largest);
            self.heapify(largest);
        }
    }

    /// Sort the the nodes in the heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use alda::heap::{Heap, Node};
    ///
    /// let mut h = Heap::from([0, -1, 7, -3].iter().map(|&x| Node::new(x)).collect(), |x: &Node<i32>, y: &Node<i32>| x < y);
    /// h.sort();
    /// assert!(h.is_sorted);
    /// ```
    ///
    pub fn sort(&mut self) {
        for i in (1..self.size).rev() {
            self.nodes.borrow_mut().swap(i, 0);
            self.size = self.size - 1;
            self.heapify(0);
        }
        self.is_sorted = true;
        self.size = self.nodes.borrow().len();
    }
}

/// Node is a node in the heap.
#[derive(Debug)]
pub struct Node<T>
where
    T: Ord,
{
    key: T,
}

impl<T> Node<T>
where
    T: Ord,
{
    /// Create a new `Node` from the given key.
    ///
    /// #  Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use alda::heap::Node;
    ///
    /// let n = Node::new(2);
    /// ```
    ///
    pub fn new(key: T) -> Node<T> {
        Node { key }
    }
}

impl<T> Eq for Node<T> where T: Ord {}

impl<T> PartialEq for Node<T>
where
    T: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> PartialOrd for Node<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Node<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}
#[cfg(test)]
use quickcheck_macros::quickcheck;

#[cfg(test)]
mod tests {
    use super::*;
    #[quickcheck]
    fn test_buid(xs: Vec<isize>) -> bool {
        if xs.len() < 2 {
            return true;
        }
        let xs = xs.iter().map(|&x| Node::new(x)).collect();
        let h = Heap::from(xs, |x, y| x > y);
        let x = h.nodes.borrow()[0] >= h.nodes.borrow()[1];
        x
    }

    #[quickcheck]
    fn test_sort_ascending(xs: Vec<isize>) -> bool {
        if xs.len() < 2 {
            return true;
        }
        let xs = xs.iter().map(|&x| Node::new(x)).collect();
        let mut h = Heap::from(xs, |x, y| x < y);
        let s = h.size;
        let mut is_sorted = false;
        h.sort();
        for i in 0..s - 1 {
            is_sorted = h.nodes.borrow()[i] >= h.nodes.borrow()[i + 1];
        }

        is_sorted
    }

    #[quickcheck]
    fn test_sort_descending(xs: Vec<isize>) -> bool {
        if xs.len() < 2 {
            return true;
        }
        let xs = xs.iter().map(|&x| Node::new(x)).collect();
        let mut h = Heap::from(xs, |x, y| x > y);
        let s = h.size;
        let mut is_sorted = false;
        h.sort();
        for i in 0..s - 1 {
            is_sorted = h.nodes.borrow()[i] <= h.nodes.borrow()[i + 1];
        }

        is_sorted
    }
}
