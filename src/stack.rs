//! Stack data structure and operations.
//!
//! This module contains a basic stack data structure and operations.
//!

/// Stack represents the stack data structure.
pub struct Stack<T> {
    items: Vec<T>,
    top: usize,
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
    /// let s = Stack::new();
    /// assert!(s.is_empty());
    /// ```
    ///
    pub fn new() -> Stack<T> {
        Stack {
            items: Vec::<T>::new(),
            top: 0,
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
    /// let s = Stack::from(v);
    /// assert!(!s.is_empty());
    pub fn from(items: Vec<T>) -> Stack<T> {
        if items.is_empty() {
            return Self::new();
        }
        let n = items.len() - 1;
        Stack { items, top: n }
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
    /// let mut s = Stack::from(vec![1, -9, 0, 3, 7]);
    /// assert_eq!(s.pop(), Ok(7));
    /// ```
    ///
    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.is_empty() {
            return Err("attempt to pop empty stack");
        }
        self.top -= 1;

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
    /// let mut s = Stack::new();
    ///
    /// ```
    ///
    pub fn push(&mut self, value: T) -> Result<(), &'static str> {
        if self.top == self.items.len() - 1 {
            return Err("the stack is already full");
        }

        self.items.push(value);
        self.top += 1;
        Ok(())
    }
}
