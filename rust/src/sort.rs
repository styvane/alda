//! Sorting algorithms
//!
//! This module implements various sorting algorithms.

use crate::Container;

/// Sort trait
///
/// The [`Sort`] trait defines the various mechanism for sorting a
/// container.
pub trait Sort<T> {
    /// Cormen, Leiserson, Rivest, and Stein insertion sort algorithm.
    ///
    /// Sort the elements in the container using CLRS insertion sort
    /// algorithm in 3rd Edition.
    fn clrs_insertion(&mut self, compare: impl Fn(&T, &T) -> bool);

    /// Alternative version of CLRS insertion algorithm.
    fn insertion(&mut self, compare: impl Fn(&T, &T) -> bool);
}

impl<T> Sort<T> for Container<T>
where
    T: Eq + PartialEq + PartialOrd + Ord + Clone,
{
    fn clrs_insertion(&mut self, compare: impl Fn(&T, &T) -> bool) {
        if self.len() <= 1 {
            return;
        }
        for j in 1..self.len() {
            let key = self[j].clone();
            let mut i = j - 1;

            while compare(&self[i], &key) {
                self.swap(i + 1, i);
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }
    }

    fn insertion(&mut self, compare: impl Fn(&T, &T) -> bool) {
        if self.len() <= 1 {
            return;
        }
        for j in 1..self.len() {
            let key = self[j].clone();
            for i in (0..j).rev() {
                if compare(&self[i], &key) {
                    self.swap(i, i + 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    use super::*;

    impl Arbitrary for Container<i32> {
        fn arbitrary(g: &mut Gen) -> Self {
            let data = Vec::<i32>::arbitrary(g);
            Self { data }
        }
    }

    #[quickcheck]
    fn test_clrs_insertion_sort_ascending(mut container: Container<i32>) -> bool {
        let mut data = container.data.clone();
        data.sort();
        container.clrs_insertion(|a, b| a > b);
        Container { data } == container
    }

    #[quickcheck]
    fn test_clrs_insertion_sort_descending(mut container: Container<i32>) -> bool {
        let mut data = container.data.clone();
        data.sort_by(|a, b| b.cmp(a));
        container.clrs_insertion(|a, b| a < b);
        Container { data } == container
    }

    #[quickcheck]
    fn insertion_sort_ascending(mut container: Container<i32>) -> bool {
        let mut data = container.data.clone();
        data.sort();
        container.insertion(|a, b| a > b);
        Container { data } == container
    }

    #[quickcheck]
    fn insertion_sort_descending(mut container: Container<i32>) -> bool {
        let mut data = container.data.clone();
        data.sort_by(|a, b| b.cmp(a));
        container.insertion(|a, b| a < b);
        Container { data } == container
    }
}
