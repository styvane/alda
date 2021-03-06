//! Circular queue data structure and operation
//!
//! This module contains basic queue data structure and operation.

/// Queue represents the queue data structure.
#[derive(Clone)]
pub struct Queue<T> {
    elements: Vec<T>,
    head: usize,
    tail: usize,
    cap: usize,
}

impl<T: Clone> Queue<T> {
    /// Create new empty queue of a given capacity.
    pub fn new(capacity: usize) -> Queue<T> {
        Queue {
            elements: Vec::with_capacity(capacity),
            head: 0,
            tail: 0,
            cap: capacity,
        }
    }

    /// Create a queue from an existing vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use alda::queue::Queue;
    /// let q = Queue::from(vec![3, 1, 0, 5], 10);
    /// assert_eq!(q.capacity(), 10);
    /// ```
    ///
    pub fn from(elements: Vec<T>, capacity: usize) -> Queue<T> {
        assert!(capacity >= elements.len());
        let tail = elements.len();
        Queue {
            elements,
            head: 0,
            tail,
            cap: capacity,
        }
    }

    /// Return the capacity of the queue.
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Check if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Insert an element in the queue.
    ///
    /// # Examples:
    ///
    /// Basic usage:
    ///
    ///```
    /// use alda::queue::Queue;
    /// let mut q = Queue::new(1);
    /// assert_eq!(q.len(), 0);
    /// q.enqueue(9);
    /// assert_eq!(q.len(), 1);
    /// ```
    ///
    pub fn enqueue(&mut self, element: T) {
        if self.len() == self.tail || self.is_empty() {
            self.elements.push(element);
        } else {
            self.elements[self.tail] = element;
        }
        if self.tail == self.cap {
            self.tail = 1;
        } else {
            self.tail += 1;
        }
    }

    /// Return the tail of the queue.
    pub fn get_tail(&self) -> usize {
        self.tail
    }

    /// Get the number of elements in the queue.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Remove the element at the head of the queue.
    pub fn dequeue(&mut self) -> Option<T> {
        let x = self.elements.get(self.head).cloned();

        if self.head == self.len() {
            self.head = 0;
        } else {
            self.head += 1;
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue() {
        let mut q: Queue<isize> = Queue::new(1);
        assert_eq!(q.get_tail(), 0);
        q.enqueue(1);
        assert_eq!(q.get_tail(), 1);
    }

    #[test]
    fn test_dequeue() {
        let mut q: Queue<isize> = Queue::new(1);
        q.enqueue(2);
        q.enqueue(1);
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), Some(1));
    }
}
