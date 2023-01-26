//! Stack data structures
//!
//! This module implements various stack data structures.

/// SimpleStack is a stack as a wrapper around vector.
#[derive(Clone, Debug, Default)]
pub struct SimpleStack<T> {
    /// Buffer data.
    buf: Vec<T>,
    /// The position of the next element on the stack.
    top: usize,
}

impl<T> SimpleStack<T> {
    /// Pushes an element onto the stack.
    pub fn push(&mut self, elem: T) {
        self.buf.push(elem);
        self.top += 1;
    }

    /// Returns true if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Pop an element from the stack.
    pub fn pop(&mut self) -> Option<T> {
        // Actually the body of this method can be replaced by
        // `self.buf.pop()` but we are sticking with CLRS algorithm.
        if self.is_empty() {
            return None;
        }
        self.top -= 1;
        Some(self.buf.remove(self.top))
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleStack;

    #[test]
    fn test_simple_stack() {
        let mut stack = SimpleStack::default();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.top, 3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.top, 1);
    }
}
