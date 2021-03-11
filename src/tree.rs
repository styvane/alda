//! Tree data structures.
//!
//! This module contains different tree data structures and the
//! operations that can be performed on them.

use std::cell::RefCell;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::io::Write;
use std::rc::{Rc, Weak};

/// BinaryTree represents a binary tree data structure.
pub struct BinaryTree<T>
where
    T: Ord + fmt::Debug + Clone + Ord,
{
    pub root: Option<Rc<RefCell<Node<T>>>>,
}

/// Child is a root or subtree root child.
type Child<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T> BinaryTree<T>
where
    T: Ord + fmt::Debug + Clone + Ord,
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
            print(&node.borrow(), out)
        }
    }

    /// Iteratively print all the keys of the nodes in the tree.
    pub fn interative_print<W: Write>(&self, out: &mut W) {
        let mut stack = vec![];
        let mut current = self.root.clone();

        loop {
            if let Some(node) = current {
                stack.push(node.borrow().clone());
                current = node.borrow().left.clone();
            } else if let Some(node) = stack.pop() {
                if let Err(e) = write!(out, "{:?}", &node.key) {
                    eprintln!("{:?}", e);
                    return;
                }
                current = node.right;
            } else {
                break;
            }
        }
    }

    /// Return the first node with the given key.
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// use alda::tree::{BinaryTree, Node};
    ///
    /// let mut tree = BinaryTree::new();
    /// let root = Some(Node::new(9));
    /// tree.root = root;
    ///```
    pub fn search(&self) -> Option<Rc<RefCell<Node<T>>>> {
        None
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
    /// Create new root or subtree root node.
    pub fn new(key: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            key,
            left: None,
            right: None,
            parent: None,
        }))
    }
}

impl<T> Eq for Node<T> where T: Ord + fmt::Debug + Clone {}

impl<T> Ord for Node<T>
where
    T: Ord + fmt::Debug + Clone,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<T> PartialEq for Node<T>
where
    T: Ord + fmt::Debug + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        &self.key == &other.key
    }
}

impl<T> PartialOrd for Node<T>
where
    T: Ord + fmt::Debug + Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_keys() {
        let mut tree: BinaryTree<isize> = BinaryTree::new();

        let root = Node::new(9);

        let left = Node::new(7);
        left.borrow_mut().parent = Some(Rc::downgrade(&root));

        root.borrow_mut().left = Some(left.clone());
        tree.root = Some(root.clone());

        let mut out = Vec::<u8>::new();
        tree.print_keys(&mut out);
        assert_eq!(String::from_utf8(out).unwrap().len(), 2);
    }

    #[test]
    fn test_iterative_print_keys() {
        let mut tree = BinaryTree::new();

        let root = Node::new(3);

        root.borrow_mut().right = Some(Node::new(4));
        let left = Node::new(2);
        left.borrow_mut().parent = Some(Rc::downgrade(&root));

        root.borrow_mut().left = Some(left.clone());
        tree.root = Some(root);

        let mut out: Vec<u8> = vec![];
        tree.interative_print(&mut out);
        assert_eq!(String::from_utf8(out).unwrap(), "234");
    }

    // #[test]
    // fn test_search() {
    //     let mut tree = BinaryTree::new();
    //     let mut root = Node::new(5);
    //     let mut left = Node::new(4);
    //     let parent = &Rc::new(RefCell::new(root));
    //     left.parent = Some(Rc::downgrade(parent));
    //     left.left = Some(Rc::new(RefCell::new(Node::new(3))));
    //     root.left = Some(Rc::new(RefCell::new(left)));
    //     let mut right = Node::new(6);
    //     right.left = Some(Rc::new(RefCell::new(Node::new(2))));
    //     root.right = Some(Rc::new(RefCell::new(right)));
    //     Some(Rc::downgrade(parent));
    //     tree.root = Some(root);

    // assert_eq!(tree.search(2).unwrap().parent.unwrap().borrow().key, 6);
    // assert_eq!(tree.search(4).unwrap().left.unwrap().borrow().key, 3);
    // }
}
