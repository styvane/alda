//! Queue data structures.

/// BoundedCircularQueue is a circular queue implemented using Vector.
#[derive(Debug, Clone, Default)]
pub struct BoundedCircularQueue<T> {
    /// The position of the element to dequeue.
    head: usize,

    /// The data buffer.
    buf: Vec<T>,

    /// Next element position.
    tail: usize,

    /// The maximum size of the queue.
    len: usize,
}

impl<T> BoundedCircularQueue<T> {
    /// Creates new queue of a maximum len.
    pub fn with_capacity(len: usize) -> Self {
        Self {
            head: 0,
            tail: 0,
            buf: Vec::with_capacity(len),
            len,
        }
    }

    /// Return true if the queue is empty.
    pub const fn is_empty(&self) -> bool {
        self.tail == self.head
    }

    /// Return true if the queue is full.
    pub const fn is_full(&self) -> bool {
        self.head == self.tail + 1 || self.head == 0 && self.tail == self.len - 1
    }
}
