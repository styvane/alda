//! Tree data structures.
//!
//! This module contains different tree data structures and the
//! operations that can be performed on them.

use std::cell::RefCell;
use std::fmt;
use std::io::Write;
use std::rc::{Rc, Weak};

/// BinaryTree represents a binary tree data structure.
pub struct BinaryTree<T: Ord + fmt::Debug> {
    root: Option<Node<T>>,
}

type Child<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T: Ord + fmt::Debug> BinaryTree<T> {
    /// Create a new binary tree.
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn print_keys<W: Write>(&self, out: &mut W) {
        fn print<T: Ord + fmt::Debug, W: Write>(node: &Node<T>, out: &mut W) {
            if let Some(ref left_child) = node.left {
                print(&left_child.borrow(), out);
            }

            if let Err(e) = write!(out, "{:?} ", &node.key) {
                eprintln!("{}", e);
                return;
            }
            if let Some(right_child) = &node.right {
                print(&right_child.borrow(), out);
            }
        }

        if let Some(ref node) = self.root {
            print(node, out)
        }
    }
}

/// Node represents a node in the binary tree.
pub struct Node<T: Ord + fmt::Debug> {
    key: T,
    left: Child<T>,
    right: Child<T>,
    parent: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Ord + fmt::Debug> Node<T> {
    /// Create new node.
    pub fn new(key: T) -> Self {
        Node {
            key,
            left: None,
            right: None,
            parent: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_keys() {
        let mut tree: BinaryTree<isize> = BinaryTree::new();
        let mut node = Node::new(9);
        node.left = Some(Rc::new(RefCell::new(Node::new(7))));
        tree.root = Some(node);
        let mut out = Vec::<u8>::new();
        tree.print_keys(&mut out);
        assert_eq!(String::from_utf8(out).unwrap().len(), 4);
    }
}
