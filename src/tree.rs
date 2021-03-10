//! Tree data structures.
//!
//! This module contains different tree data structures and the
//! operations that can be performed on them.

use std::cell::RefCell;
use std::fmt;
use std::io::Write;
use std::rc::{Rc, Weak};

/// BinaryTree represents a binary tree data structure.
pub struct BinaryTree<T>
where
    T: Ord + fmt::Debug + Clone,
{
    root: Option<Node<T>>,
}

/// Child is a root or subtree root child.
type Child<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T> BinaryTree<T>
where
    T: Ord + fmt::Debug + Clone,
{
    /// Create a new binary tree.
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Recursively print all the keys of the nodes in the tree.
    pub fn print_keys<W: Write>(&self, out: &mut W) {
        fn print<T: Ord + fmt::Debug + Clone, W: Write>(node: &Node<T>, out: &mut W) {
            if let Some(ref left_child) = node.left {
                print(&left_child.borrow(), out);
            }

            if let Err(e) = write!(out, "{:?}", &node.key) {
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

    /// Iteratively print all the keys of the nodes in the tree.
    pub fn interative_print<W: Write>(&self, out: &mut W) {
        let mut stack = vec![];
        let mut current = self.root.clone();
        loop {
            if let Some(ref node) = current {
                stack.push(node.clone());
                if let Some(ref left_child) = node.left {
                    let child = left_child.borrow().clone();
                    current = Some(child);
                } else {
                    current = None;
                }
            } else if let Some(node) = stack.pop() {
                if let Err(e) = write!(out, "{:?}", &node.key) {
                    eprintln!("{:?}", e);
                    return;
                }

                if let Some(ref right_child) = node.right {
                    current = Some(right_child.borrow().clone());
                } else {
                    current = None;
                }
            } else {
                break;
            }
        }
    }
}

/// Node represents a node in the binary tree.
#[derive(Clone, Debug)]
pub struct Node<T>
where
    T: Ord + fmt::Debug + Clone,
{
    key: T,
    left: Child<T>,
    right: Child<T>,
    parent: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Ord + fmt::Debug + Clone,
{
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
        assert_eq!(String::from_utf8(out).unwrap().len(), 2);
    }

    #[test]
    fn test_iterative_print_keys() {
        let mut tree: BinaryTree<isize> = BinaryTree::new();
        let mut root = Node::new(3);
        root.left = Some(Rc::new(RefCell::new(Node::new(2))));
        root.right = Some(Rc::new(RefCell::new(Node::new(4))));
        root.left = root.left.map(|x| {
            x.borrow_mut().left = Some(Rc::new(RefCell::new(Node::new(1))));
            x
        });
        tree.root = Some(root);
        let mut out: Vec<u8> = vec![];
        tree.interative_print(&mut out);
        assert_eq!(String::from_utf8(out).unwrap().len(), 4);
    }
}
