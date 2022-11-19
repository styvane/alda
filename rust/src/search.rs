//! Searching algorithms.
//!
//! Defines module implements various searching algorithms.

use std::cmp::Ordering;

use crate::Container;

/// The [`Search`] trait specifies the various methods for searching.
pub trait Search<T> {
    /// Search linearly for a value in a container.
    ///
    /// It returns the position of the value if it exists.
    fn linear(&self, needle: T) -> Option<usize>;

    /// Binary searches for value in a sorted container.
    fn binsearch(&self, needle: &T) -> Option<usize>;

    /// Recursively binary search for a value in a sorted container.
    fn rec_binsearch(&self, needle: &T) -> Option<usize>;
}

impl<T> Search<T> for Container<T>
where
    T: PartialEq + PartialOrd + Clone + Ord,
{
    fn linear(&self, needle: T) -> Option<usize> {
        for (index, value) in self.iter().enumerate() {
            if *value == needle {
                return Some(index);
            }
        }
        None
    }

    fn binsearch(&self, needle: &T) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        let (mut low, mut high) = (0, self.len() - 1);

        while low <= high {
            let middle = (low + high) / 2;
            match &self[middle].cmp(needle) {
                Ordering::Less => {
                    high = middle + 1;
                }
                Ordering::Greater => {
                    low = middle - 1;
                }
                Ordering::Equal => return Some(middle),
            }
        }
        None
    }

    fn rec_binsearch(&self, needle: &T) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        fn rec<T: PartialEq + PartialOrd + Eq>(haystack: &[T], needle: &T) -> Option<usize> {
            let middle = haystack.len() / 2;
            if *needle == haystack[middle] {
                Some(middle)
            } else if *needle > haystack[middle] {
                rec(&haystack[middle + 1..], needle)
            } else if *needle < haystack[middle] {
                rec(&haystack[..middle], needle)
            } else {
                None
            }
        }
        rec(&self.data, needle)
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

    #[test]
    fn binsearch_missing_value_return_none() {
        let container = Container {
            data: vec![1, 2, 3, 4, 5],
        };
        assert!(
            container.binsearch(&7).is_none(),
            "failed to return the index"
        );
    }

    #[test]
    fn binsearch_existing_value_return_the_index() {
        let container = Container {
            data: vec![1, 2, 3, 4, 5],
        };
        assert_eq!(
            container.binsearch(&3),
            Some(2),
            "failed to return the index"
        );
    }

    #[test]
    fn rec_binsearch_existing_value_return_the_index() {
        let container = Container {
            data: vec![1, 2, 3, 4, 5],
        };
        assert_eq!(
            container.rec_binsearch(&3),
            Some(2),
            "failed to return the index"
        );
    }
}
