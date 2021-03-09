//! Double Linked list implementation.
//!
//! This module contains a double linked list operations.

use std::cmp::Ordering;
use std::fmt::Debug;
use std::ptr::NonNull;

/// LinkedList represents a linked list data structure.
#[derive(Debug)]
pub struct LinkedList<T: Ord + Debug> {
    head: Option<NonNull<Node<T>>>,
    size: usize,
}

impl<T: Ord + Debug> LinkedList<T> {
    /// Create a new empty link list.
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    /// Find the first node with the given key in the linked list.
    ///
    /// # Examples
    ///
    /// ```
    /// use alda::linkedlist::{LinkedList, Node};
    ///
    /// let link: LinkedList<isize> = LinkedList::new();
    /// let x = 4;
    /// assert!(link.search(&x).is_none());
    ///```
    ///
    pub fn search(&self, key: &T) -> Option<&Node<T>> {
        let mut link = &self.head;
        while let Some(node) = link {
            unsafe {
                if &node.as_ref().key != key {
                    link = &(*node.as_ptr()).next;
                } else {
                    return Some(node.as_ref());
                }
            }
        }
        None
    }

    /// Insert a new node with the given key into the linked list.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use alda::linkedlist::{LinkedList, Node};
    ///
    /// let mut link = LinkedList::new();
    /// link.insert(Node::new(2));
    /// assert!(link.search(&2).is_some());
    /// assert!(link.search(&3).is_none());
    /// ```
    pub fn insert(&mut self, mut node: Node<T>) {
        node.next = self.head;
        if let Some(head) = self.head {
            unsafe {
                (*head.as_ptr()).prev = NonNull::new(&mut node as *mut Node<T>);
            }
        }
        self.head = NonNull::new(&mut node as *mut Node<T>);
        node.prev = None;
        self.size += 1;
    }

    /// Return the length of the linked list.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use alda::linkedlist::{LinkedList, Node};
    ///
    /// let mut link: LinkedList<isize> = LinkedList::new();
    /// assert_eq!(link.len(), 0);
    /// link.insert(Node::new(1));
    /// assert_eq!(link.len(), 1);
    /// ```
    ///
    pub fn len(&self) -> usize {
        self.size
    }

    /// Delete a node from the linked list.
    ///
    /// # Safety
    /// The node must be in the linked list.
    ///
    pub fn delete(&mut self, node: &Node<T>) -> Result<(), &'static str> {
        if let Some(prev) = node.prev {
            unsafe {
                (*prev.as_ptr()).next = node.next;
            }
        } else {
            self.head = node.next;
        }

        if let Some(ref next) = node.next {
            unsafe {
                (*next.as_ptr()).prev = node.prev;
            }
        }
        Ok(())
    }
}

/// Node is a node in the linked list
#[derive(Debug, Eq)]
pub struct Node<T: Ord + Debug> {
    key: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T: Ord + Debug> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<T: Ord + Debug> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord + Debug> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl<T: Ord + Debug> Node<T> {
    /// Create a new node.
    pub fn new(key: T) -> Node<T> {
        Node {
            key,
            next: None,
            prev: None,
        }
    }
}
#[cfg(test)]
use quickcheck_macros::quickcheck;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_empty() {
        let link: LinkedList<isize> = LinkedList::new();
        assert_eq!(link.search(&0), None);
    }

    #[test]
    fn test_does_not_contain_node() {
        let mut link = LinkedList::new();
        link.insert(Node::new(2));
        assert!(link.search(&9).is_none());
    }

    #[quickcheck]
    fn test_node_found(xs: Vec<isize>) -> bool {
        let mut vc = xs.clone();
        vc.reverse();
        let mut link = LinkedList::new();
        xs.iter().for_each(|&x| link.insert(Node::new(x)));
        vc.iter()
            .for_each(|x| assert_eq!(link.search(x).unwrap().key, *x));
        true
    }

    #[test]
    fn test_insert_one_node() {
        let mut link = LinkedList::new();
        assert_eq!(link.len(), 0);
        link.insert(Node::new(7));
        assert_eq!(link.len(), 1);
    }

    #[quickcheck]
    fn test_insert_many_node(xs: Vec<isize>) -> bool {
        let mut lst = LinkedList::new();
        xs.iter().for_each(|&x| lst.insert(Node::new(x)));
        true
    }
}
