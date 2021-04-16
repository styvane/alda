//! Heap data structure routine
//!
//! This module implement the [`Heap`] data structure and the
//! associated methods or functions.
//!

use std::cell::RefCell;
use std::fmt;

/// The [`Kind`] type represents the heap type.
#[derive(Debug, Clone)]
pub enum Kind {
    Min,
    Max,
}

/// Heap is a binary heap data structure.
#[derive(Clone)]
pub struct Heap<T> {
    pub size: usize,
    pub is_sorted: bool,

    kind: Kind,
    nodes: RefCell<Vec<Node<T>>>,
}

impl<T> fmt::Debug for Heap<T>
where
    T: Ord + fmt::Debug + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Heap").field("nodes", &self.nodes).finish()
    }
}

impl<T> Heap<T>
where
    T: Ord + Clone,
{
    /// Create a new `Heap`.

    ///
    /// # Examples
    ///
    /// ```
    /// use alda::heap::{Heap, Node, Kind};
    ///
    /// let h: Heap<Node<i32>> = Heap::new(Kind::Max);
    /// ```
    ///
    pub fn new(kind: Kind) -> Heap<T> {
        Heap {
            size: 0,
            kind,
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
    /// use alda::heap::{Heap, Node, Kind};
    ///
    /// let h = Heap::from(vec![Node::new(1)], Kind::Max);
    ///
    pub fn from(nodes: Vec<Node<T>>, kind: Kind) -> Heap<T> {
        assert!(nodes.len() > 0);

        let mut h = Heap {
            kind,
            size: nodes.len(),
            is_sorted: false,
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
    /// use alda::heap::{Heap, Node, Kind};
    ///
    /// let mut h: Heap<Node<i32>> = Heap::new(Kind::Max);
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
        let mut predicate: Box<dyn Fn(&Node<T>, &Node<T>) -> bool> = Box::new(|x, y| x > y);
        if let Kind::Min = self.kind {
            predicate = Box::new(|x, y| x < y);
        }
        if left < self.size && (predicate)(&self.nodes.borrow()[left], &self.nodes.borrow()[index])
        {
            largest = left;
        }

        let right = 2 * index + 2;
        if right < self.size
            && (predicate)(&self.nodes.borrow()[right], &self.nodes.borrow()[largest])
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
    /// If the heap is a min-heap, the nodes are sorted in non increasing order,
    /// otherwise, they are sorted in increasing order.
    /// After this operation, the heap property is no longer maintained.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use alda::heap::{Heap, Node, Kind};
    ///
    /// let nodes = [0, -1, 7, -3].iter()
    ///     .map(|&x| Node::new(x))
    ///     .collect();
    /// let mut h = Heap::from(nodes, Kind::Max);
    ///
    /// h.sort(); // sort nodes in non increasing order
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
        self.update_size();
    }

    /// Return the minimum value in the heap.
    ///
    /// The minimum value for a min-heap or sorted max-heap is returned in O(1) time,
    /// however on a max-heap, the worst case is O(m * n * log(n)).
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use alda::heap::{Heap, Node, Kind};
    /// let nodes = [0, -1, 7, -3].iter()
    ///     .map(|&x| Node::new(x))
    ///     .collect();
    /// let mut h = Heap::from(nodes, Kind::Max);
    /// h.sort();
    /// assert_eq!(h.min(), Some(Node{key:-3}));
    /// ```
    ///
    pub fn min(&self) -> Option<Node<T>> {
        if self.size == 0 {
            return None;
        }
        if let Kind::Min = self.kind {
            if self.is_sorted {
                self.nodes.borrow().last().cloned()
            } else {
                self.nodes.borrow().first().cloned()
            }
        } else {
            if self.is_sorted {
                self.nodes.borrow().first().cloned()
            } else {
                self.nodes
                    .borrow()
                    .iter()
                    .min_by_key(|&x| x.clone().key)
                    .cloned()
            }
        }
    }

    /// Return the maximum value in the [`Heap`].
    ///
    /// The maximum for a max-heap or sorted min-heap is returned in O(1) time,
    /// however on a min-heap, the worst case is O(m * n * log(n)).
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use alda::heap::{Node, Kind, Heap};
    ///
    /// let nodes = [0, -17, -1, 9].iter().map(|&x| Node::new(x)).collect();
    /// let mut h = Heap::from(nodes, Kind::Max);
    /// assert_eq!(h.max(), Some(Node {key: 9}));
    /// ```
    pub fn max(&self) -> Option<Node<T>> {
        if self.size == 0 {
            return None;
        }
        if let Kind::Min = self.kind {
            if self.is_sorted {
                self.nodes.borrow().first().cloned()
            } else {
                self.nodes
                    .borrow()
                    .iter()
                    .max_by_key(|&x| x.clone().key)
                    .cloned()
            }
        } else {
            if self.is_sorted {
                self.nodes.borrow().last().cloned()
            } else {
                self.nodes.borrow().first().cloned()
            }
        }
    }

    /// Update heap size.
    fn update_size(&mut self) {
        self.size = self.nodes.borrow().len();
    }

    /// Extract the maximum value in the [`Heap`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use alda::heap::{Heap, Node, Kind};
    ///
    /// let mut h = Heap::from(
    ///     [-9, 0, 7 ,1].iter().map(|&x| Node::new(x)).collect(),
    ///     Kind::Max);
    /// let max = h.extract_max();
    /// ```
    ///
    pub fn extract_max(&mut self) -> Option<Node<T>> {
        if self.size == 0 {
            return None;
        }
        if let Kind::Min = self.kind {
            if self.is_sorted {
                Some(self.nodes.borrow_mut().remove(0))
            } else {
                let idx = self
                    .nodes
                    .borrow()
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, val)| val.key.clone())
                    .map(|(idx, _)| idx);

                let max = Some(self.nodes.borrow_mut().swap_remove(idx.unwrap()));
                self.update_size();
                self.build();
                max
            }
        } else {
            if self.is_sorted {
                self.nodes.borrow_mut().pop()
            } else {
                let max = Some(self.nodes.borrow_mut().swap_remove(0));
                self.update_size();
                self.heapify(0);
                max
            }
        }
    }

    /// Increasse a node key in max heap while maintaining the heap property.
    ///
    /// # Panics
    ///
    /// This method will panic if the index of the node is greater than the heap size.
    ///
    pub fn increase_key(&mut self, key: T, position: usize) -> Result<(), &'static str> {
        assert!(position < self.size);
        if key < self.nodes.borrow()[position].key {
            return Err("new key smaller than current key");
        }

        let mut pos = position;
        self.nodes.borrow_mut()[pos] = Node::new(key);
        let parent = position / 2;
        while pos > 0 && self.nodes.borrow()[parent] < self.nodes.borrow()[pos] {
            self.nodes.borrow_mut().swap(pos, parent);
            pos = parent;
        }
        Ok(())
    }
}

/// Node is a node in the heap.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Node<T> {
    pub key: T,
}

impl<T> Node<T> {
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
        let h = Heap::from(xs, Kind::Max);
        let x = h.nodes.borrow()[0] >= h.nodes.borrow()[1];
        x
    }

    #[quickcheck]
    fn test_sort_ascending(xs: Vec<isize>) -> bool {
        if xs.len() < 2 {
            return true;
        }
        let xs = xs.iter().map(|&x| Node::new(x)).collect();
        let mut h = Heap::from(xs, Kind::Min);
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
        let mut h = Heap::from(xs, Kind::Max);
        let s = h.size;
        let mut is_sorted = false;
        h.sort();
        for i in 0..s - 1 {
            is_sorted = h.nodes.borrow()[i] <= h.nodes.borrow()[i + 1];
        }

        is_sorted
    }

    #[test]
    fn test_min() {
        let mut h = Heap::from(
            [1, -9, 11, -3, 0, 7]
                .iter()
                .map(|&x| Node::new(x))
                .collect(),
            Kind::Min,
        );
        assert_eq!(h.min(), Some(Node { key: -9 }));
        h.sort();
        assert_eq!(h.min(), Some(Node { key: -9 }));
    }

    #[test]
    fn test_min_max_heap() {
        let mut h = Heap::from(
            [1, -9, 11, -3, 0, 7]
                .iter()
                .map(|&x| Node::new(x))
                .collect(),
            Kind::Max,
        );
        assert_eq!(h.min(), Some(Node { key: -9 }));
        h.sort();
        assert_eq!(h.min(), Some(Node { key: -9 }));
    }

    #[test]
    fn test_max() {
        let mut h = Heap::from(
            [1, -21, 17, 0, 11, 4]
                .iter()
                .map(|&x| Node::new(x))
                .collect(),
            Kind::Min,
        );

        assert_eq!(h.max(), Some(Node { key: 17 }));
        h.sort();
        assert_eq!(h.max(), Some(Node { key: 17 }));
    }

    #[test]
    fn test_extract_max() {
        let mut h = Heap::from(
            [-7, 9, 0, 3].iter().map(|&x| Node::new(x)).collect(),
            Kind::Min,
        );

        let max = h.extract_max();
        assert_eq!(max, Some(Node { key: 9 }));
        assert_eq!(h.size, 3);
    }
}
