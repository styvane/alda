//! Circular queue data structure and operation
//!
//! This module contains basic queue data structure and operation.

/// Queue represents the queue data structure.
pub struct Queue<T> {
    elements: Vec<T>,
    head: usize,
    tail: usize,
    pub length: usize,
}

impl<T> Queue<T> {
    /// Create new empty queue.
    pub fn new() -> Queue<T> {
        Queue {
            elements: vec![],
            head: 0,
            tail: 0,
            length: 0,
        }
    }

    /// Create a queue from an existing vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use alda::queue::Queue;
    /// let q = Queue::from(vec![3, 1, 0, 5]);
    /// assert_eq!(q.length, 4);
    /// ```
    ///
    pub fn from(elements: Vec<T>) -> Queue<T> {
        let length = elements.len();
        let tail = elements.len();
        Queue {
            elements,
            head: 0,
            tail,
            length,
        }
    }
}
