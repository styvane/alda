//! Queue data structures.

use crate::{error::ErrorKind, Error};

/// BoundedQueue is a circular queue implemented using a vector.
#[derive(Debug, Clone, Default)]
pub struct BoundedQueue<T> {
    /// The position of the element to dequeue.
    head: usize,

    /// The data buffer.
    buf: Vec<T>,

    /// Then position of the next element to enqueue.
    tail: usize,

    /// The maximum size of the queue.
    capacity: usize,

    /// The number of element in the queue.
    len: usize,
}

impl<T> BoundedQueue<T>
where
    T: PartialEq + Clone,
{
    /// Create new queue with the given maximum capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            head: 0,
            tail: 0,
            len: 0,
            buf: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// Return true if the queue is empty.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Return true if the queue is full.
    // The queue is full if `tail` point to the capacity or
    // the tail point to one element before the head.
    pub const fn is_full(&self) -> bool {
        self.len == self.capacity
    }

    /// Insert new element to the queue.
    pub fn enqueue(&mut self, elem: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::new(ErrorKind::QueueOverflow));
        }

        if let Some(val) = self.buf.get_mut(self.tail) {
            *val = elem;
        } else {
            self.buf.insert(self.tail, elem);
        }

        self.tail = (self.tail + 1) % self.capacity;
        self.len += 1;

        Ok(())
    }

    /// Delete an element from the queue.
    pub fn dequeue(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::new(ErrorKind::QueueUnderflow));
        }
        let val = self.buf[self.head].clone();
        self.len -= 1;
        self.head = (self.head + 1) % self.capacity;
        Ok(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(unused_must_use)]
    fn bounded_queue() {
        let mut queue = BoundedQueue::<i32>::with_capacity(5);
        assert!(queue.is_empty());
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);
        queue.enqueue(5);
        assert!(queue.is_full());
        assert_eq!(queue.tail, 0);
        assert!(queue.enqueue(6).is_err());
        assert_eq!(queue.dequeue(), Ok(1));
        assert_eq!(queue.head, 1);
        queue.enqueue(6);
        assert!(queue.is_full());
        assert_eq!(queue.tail, 1);
        assert_eq!(queue.dequeue(), Ok(2));
        assert_eq!(queue.dequeue(), Ok(3));
        assert_eq!(queue.dequeue(), Ok(4));
        assert_eq!(queue.dequeue(), Ok(5));
    }
}
