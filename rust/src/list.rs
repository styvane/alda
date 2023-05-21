//! Linked lists data structure

/// A List is a single linked list data structure.
#[derive(Debug, Clone)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

/// Node represents a node in the linked-list.
#[derive(Debug, Clone)]
pub struct Node<T> {
    /// The node key value.
    pub key: T,

    /// `next` points to the next node.
    next: Link<T>,
}

impl<T> Node<T> {
    /// create new node with the given key.
    const fn with_key(key: T) -> Self {
        Self { key, next: None }
    }
}

impl<T> List<T> {
    /// Create new list with the given key.
    pub const fn new() -> Self {
        Self { head: None }
    }

    /// Inserts new node into the list.
    ///
    /// It inserts the new node into the list by splicing the list
    /// on the head.
    pub fn insert(&mut self, key: T) -> &mut Self {
        let mut node = Box::new(Node::with_key(key));
        node.next = self.head.take();
        self.head = Some(node);
        self
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn list_operations() {
        let mut list = List::default();
        list.insert(1);
        list.insert(2);
        list.insert(3);
        list.insert(4);
    }
}
