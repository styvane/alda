//! Binary Tree data structures.
//!
//! This module is an attempt to implement the binary tree data structure.

use std::cell::RefCell;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::io::Write;
use std::rc::{Rc, Weak};

/// BinaryTree represents a binary tree data structure.
pub struct BinaryTree<T>
where
    T: Ord + fmt::Debug + Clone,
{
    pub root: Option<Rc<RefCell<Node<T>>>>,
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
    /// use alda::tree::binary_tree::{BinaryTree, Node};
    ///
    /// let mut tree = BinaryTree::new();
    /// let root = Some(Node::new(9));
    /// tree.root = root;
    ///```
    pub fn search(&self, key: &T) -> Option<Rc<RefCell<Node<T>>>> {
        fn find<T: Ord + fmt::Debug + Clone>(
            key: &T,
            node: Rc<RefCell<Node<T>>>,
        ) -> Option<Rc<RefCell<Node<T>>>> {
            if &node.borrow().key == key {
                return Some(node);
            } else if &node.borrow().key > key {
                if let Some(left) = node.borrow().left.clone() {
                    return find(&key, left);
                }
            } else {
                if let Some(right) = node.borrow().right.clone() {
                    return find(&key, right);
                }
            }

            None
        }

        if let Some(ref root) = self.root {
            return find(&key, root.clone());
        }
        None
    }

    /// Iteratively search for a node with the given key.
    pub fn iterative_search(&self, key: &T) -> Option<Rc<RefCell<Node<T>>>> {
        if self.root.is_none() {
            return None;
        }
        let mut current = self.root.clone().unwrap();
        loop {
            if &current.borrow().key == key {
                return Some(current);
            } else if key < &current.borrow().key {
                let lc = current.borrow().left.clone();
                if let Some(left) = lc {
                    current = left;
                } else {
                    return None;
                }
            } else {
                let rc = current.borrow().right.clone();
                if let Some(right) = rc {
                    current = right;
                } else {
                    return None;
                }
            }
        }
    }

    /// Return a node with the maximum value.
    pub fn min(&self) -> Option<Rc<RefCell<Node<T>>>> {
        if self.root.is_none() {
            return None;
        }
        let mut x = self.root.clone().unwrap();
        while let Some(node) = x.clone().borrow().left.clone() {
            x = node;
        }
        Some(x)
    }

    /// Return a node with the maximum value.
    pub fn max(&self) -> Option<Rc<RefCell<Node<T>>>> {
        if self.root.is_none() {
            return None;
        }
        let mut x = self.root.clone().unwrap();
        while let Some(node) = x.clone().borrow().right.clone() {
            x = node;
        }
        Some(x)
    }

    /// Return the successor for a node with the given key.
    pub fn successor(&self, key: &T) -> Option<Rc<RefCell<Node<T>>>> {
        if let Some(node) = self.iterative_search(&key) {
            if let Some(_) = node.borrow().right {
                let mut tree = BinaryTree::new();
                tree.root = node.borrow().right.clone();
                return tree.min();
            } else {
                let mut node = node.clone();
                let mut parent = node.borrow().parent.clone();
                while let Some(ref p) = parent {
                    let ptr = p.upgrade().unwrap().as_ptr();
                    unsafe {
                        if (*ptr).right.is_none() {
                            return Some(p.upgrade().unwrap());
                        } else {
                            if let Some(ref rs) = (*ptr).right {
                                if &rs.borrow().key == key {
                                    node = p.upgrade().unwrap();
                                    parent = node.borrow().parent.clone();
                                }
                            }
                        }
                    }
                }
                return None;
            }
        } else {
            return None;
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

    fn new_tree() -> BinaryTree<isize> {
        let mut tree = BinaryTree::new();

        let root = Node::new(7);
        let left_child = Node::new(5);
        left_child.borrow_mut().parent = Some(Rc::downgrade(&root));
        let lc = Node::new(4);
        lc.borrow_mut().parent = Some(Rc::downgrade(&left_child));
        left_child.borrow_mut().left = Some(lc);

        root.borrow_mut().left = Some(left_child);

        let right = Node::new(9);
        let rc = Node::new(8);
        rc.borrow_mut().parent = Some(Rc::downgrade(&right));
        right.borrow_mut().left = Some(rc);

        root.borrow_mut().right = Some(right);

        tree.root = Some(root);
        return tree;
    }

    #[test]
    fn test_search() {
        let tree = new_tree();
        assert_eq!(
            tree.search(&8)
                .unwrap()
                .borrow()
                .parent
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow()
                .key,
            9
        );
        assert_eq!(
            tree.search(&5)
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .key,
            4
        );
    }

    #[test]
    fn test_iterative_search() {
        let tree = new_tree();
        assert_eq!(
            tree.iterative_search(&8)
                .unwrap()
                .borrow()
                .parent
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow()
                .key,
            9
        );
        assert_eq!(
            tree.iterative_search(&5)
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .key,
            4
        );
    }

    #[test]
    fn test_maximum() {
        let tree = new_tree();
        assert_eq!(tree.max().unwrap().borrow().key, 9);
    }

    #[test]
    fn test_minimum() {
        let tree = new_tree();
        assert_eq!(tree.min().unwrap().borrow().key, 4);
    }

    #[test]
    fn test_successor() {
        let tree = new_tree();
        assert_eq!(tree.successor(&8).unwrap().borrow().key, 9);
        assert!(tree.successor(&9).is_none());
    }
}
