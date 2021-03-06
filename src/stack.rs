//! Stack data structure and operations.
//!
//! This module contains a basic stack data structure and operations.
//!

/// Stack represents the stack data structure.
pub struct Stack<T> {
    items: Vec<T>,
    top: usize,
    cap: usize,
}

impl<T> Stack<T> {
    /// Create new empty stack.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use alda::stack::Stack;
    ///
    /// let s: Stack<isize> = Stack::new(0);
    /// assert!(s.is_empty());
    /// ```
    ///
    pub fn new(capacity: usize) -> Stack<T> {
        Stack {
            items: Vec::<T>::new(),
            top: 0,
            cap: capacity,
        }
    }

    /// Create new stack from the vector.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use alda::stack::Stack;
    ///
    /// let v = vec![1, 0, 2];
    /// let s = Stack::from(v, 3);
    /// assert!(!s.is_empty());
    pub fn from(items: Vec<T>, capacity: usize) -> Stack<T> {
        if items.is_empty() {
            return Self::new(capacity);
        }
        Stack {
            items,
            top: 0,
            cap: capacity,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Remove and return the item on top of the stack.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use alda::stack::Stack;
    ///
    /// let mut s = Stack::from(vec![1, -9, 0, 3, 7], 6);
    /// assert_eq!(s.pop(), Ok(7));
    /// ```
    ///
    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.is_empty() {
            return Err("attempt to pop empty stack");
        }
        if self.top > 0 {
            self.top -= 1;
        }
        Ok(self.items.pop().unwrap())
    }

    /// Push an item into the stack.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use alda::stack::Stack;
    ///
    /// let mut s = Stack::new(1);
    /// assert_eq!(s.len(), 0);
    /// s.push(1);
    /// assert_eq!(s.len(), 1)
    /// ```
    ///
    pub fn push(&mut self, value: T) -> Result<(), &'static str> {
        if self.len() == self.cap {
            return Err("the stack is already full");
        }

        self.items.push(value);
        self.top += 1;
        Ok(())
    }

    /// Return the size of the stack
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop() {
        let mut s = Stack::from(vec![-9, 1], 3);
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Ok(-9));
        assert!(s.pop().is_err());
    }

    #[test]
    fn test_push() {
        let mut s: Stack<isize> = Stack::new(1);
        assert_eq!(s.len(), 0);
        assert!(s.push(3).is_ok());
        assert!(s.push(2).is_err());
    }
}
