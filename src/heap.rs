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
    nodes: RefCell<Vec<Node<T>>>,
    predicate: P,
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
            nodes: RefCell::new(Vec::<Node<T>>::new()),
            predicate,
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
            nodes: RefCell::new(nodes),
            predicate,
        };
        h.build();
        h
    }
    /// Get a `Heap` size.
    ///
    /// # Examples
    ///
    /// ```
    /// use alda::heap::{Heap, Node};
    ///
    /// let h = Heap::from(vec![Node::new(3), Node::new(1)], |x, y| x < y);
    /// assert_eq!(h.size(), 2);
    /// ```
    ///
    pub fn size(&self) -> usize {
        self.nodes.borrow().len()
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
        if self.size() < 2 {
            return;
        }
        for i in (0..self.size() / 2).rev() {
            self.heapify(i);
        }
    }

    fn heapify(&self, index: usize) {
        let left = 2 * index + 1;
        let mut largest = index;

        if left < self.size()
            && (self.predicate)(&self.nodes.borrow()[left], &self.nodes.borrow()[index])
        {
            largest = left;
        }

        let right = 2 * index + 2;
        if right < self.size()
            && (self.predicate)(&self.nodes.borrow()[right], &self.nodes.borrow()[largest])
        {
            largest = right;
        }

        if largest != index {
            self.nodes.borrow_mut().swap(index, largest);
            self.heapify(largest);
        }
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
}
