//! Sorting algorithms
//!
//! This module implements various sorting algorithms.

use std::ops::{Index, IndexMut};

/// The [`Container`] type is a wrapper around the containing data.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub struct Container<T> {
    data: Vec<T>,
}

impl<T> Container<T> {
    /// Creates new container instance.
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    /// Returns true the container is empty, otherwise it returns false.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of items in the container.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Swaps two elements in the container.
    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }
}

impl<T> Index<usize> for Container<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl<T> IndexMut<usize> for Container<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

/// Sort trait
///
/// The [`Sort`] trait defines the various mechanism for sorting a
/// container.
pub trait Sort<E> {
    /// Cormen, Charles, Ronald and Clifford insertion sort algorithm.
    ///
    /// Sort the elements in the container using CLRS insertion sort
    /// algorithm in 3rd Edition.
    fn clrs_insertion(&mut self, compare: impl Fn(&E, &E) -> bool);

    /// Alternative version of CLRS insertion algorithm.
    fn insertion(&mut self, compare: impl Fn(&E, &E) -> bool);
}

impl<E> Sort<E> for Container<E>
where
    E: Eq + PartialEq + PartialOrd + Ord + Clone,
{
    fn clrs_insertion(&mut self, compare: impl Fn(&E, &E) -> bool) {
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

    fn insertion(&mut self, compare: impl Fn(&E, &E) -> bool) {
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
