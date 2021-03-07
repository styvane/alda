//! Stack data structure and operations.
//!
//! This module contains a basic stack data structure and operations.
//!

use std::mem;

/// Link is the list to the next element on the stack.
type Link<T> = Option<Box<Elem<T>>>;

/// Stack represents the stack data structure.
pub struct Stack<T: Ord> {
    top: Link<T>,
    cap: usize,
    pub len: usize,
}

/// Elem represents an element on the stack.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Elem<T: Ord> {
    pub key: T,
    prev: Link<T>,
}

impl<T: Ord> Stack<T> {
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
            top: None,
            cap: capacity,
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top.is_none()
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
    /// let mut s = Stack::new(2);
    /// s.push(3);
    /// s.push(1);
    /// assert_eq!(s.pop().unwrap().key, 1);
    /// ```
    ///
    pub fn pop(&mut self) -> Result<Elem<T>, &'static str> {
        if self.is_empty() {
            return Err("attempt to pop empty stack");
        }
        self.len -= 1;
        let mut v = self.top.take().unwrap();
        let prev = v.prev;
        v.prev = None;

        if let Some(prev) = prev {
            self.top = Some(prev);
        }

        Ok(*v)
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
    /// assert!(s.is_empty());
    /// s.push(1);
    /// assert_eq!(s.len, 1)
    /// ```
    ///
    pub fn push(&mut self, key: T) -> Result<(), &'static str> {
        if self.len == self.cap {
            return Err("the stack is already full");
        }

        self.len += 1;
        let old_top = mem::replace(&mut self.top, Some(Box::new(Elem { key, prev: None })));
        if let Some(old_top) = old_top {
            let mut top = self.top.take().unwrap();
            top.prev = Some(old_top);
            self.top = Some(top);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop() {
        let mut s = Stack::new(2);
        assert!(s.push(-9).is_ok());
        assert!(s.push(1).is_ok());
        assert_eq!(s.pop().unwrap().key, 1);
        assert_eq!(s.pop().unwrap().key, -9);
        assert!(s.pop().is_err());
    }

    #[test]
    fn test_push() {
        let mut s: Stack<isize> = Stack::new(1);
        assert_eq!(s.len, 0);
        assert!(s.push(3).is_ok());
        assert!(s.push(2).is_err());
    }
}
