//! Searching algorithms.
//!
//! Defines module implements various searching algorithms.

use crate::Container;

/// The [`Search`] trait specifies the various methods for searching.
pub trait Search<T> {
    /// Search linearly for a value in a container.
    ///
    /// It returns the position of the value if it exists.
    fn linear(&self, needle: T) -> Option<usize>;
}

impl<T> Search<T> for Container<T>
where
    T: PartialEq,
{
    fn linear(&self, needle: T) -> Option<usize> {
        for (index, value) in self.iter().enumerate() {
            if *value == needle {
                return Some(index);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_searching_a_value_in_empty_container_return_none() {
        let container = Container::default();
        assert!(
            container.linear(4).is_none(),
            "empty container returned index"
        )
    }

    #[test]
    fn linear_searching_existing_value_return_the_index() {
        let container = Container {
            data: vec![1, 2, 3, 4, 5],
        };
        assert_eq!(container.linear(3), Some(2), "failed to return the index");
    }

    #[test]
    fn linear_searching_missing_value_return_none() {
        let container = Container {
            data: vec![1, 2, 3, 4, 5],
        };
        assert!(container.linear(7).is_none(), "failed to return the index");
    }
}
