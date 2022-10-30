//! Sorting algorithms
//!
//! This module implements various sorting algorithms.

use std::{
    ops::{Index, IndexMut},
    usize,
};

/// The [`Container`] type is a wrapper around the containing data.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub struct Container<T> {
    data: Vec<T>,
}

impl<T> Container<T> {
    /// Return the number of items in the container.
    fn len(&self) -> usize {
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
trait Sort {
    /// Cormen, Charles, Ronald and Clifford insertion sort algorithm.
    ///
    /// Sort the elements in the container using CLRS insertion sort
    /// algorithm in 3rd Edition.
    fn clrs_insertion(&mut self);

    /// Alternative version of CLRS insertion algorithm.
    fn insertion(&mut self);
}

impl<T> Sort for Container<T>
where
    T: Eq + PartialEq + PartialOrd + Ord + Clone,
{
    fn clrs_insertion(&mut self) {
        if self.len() <= 1 {
            return;
        }
        for j in 1..self.len() {
            let key = self[j].clone();
            let mut i = j - 1;
            while self[i] > key {
                self.swap(i + 1, i);
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }
    }

    fn insertion(&mut self) {
        if self.len() <= 1 {
            return;
        }
        for j in 1..self.len() {
            let key = self[j].clone();
            for i in (0..j).rev() {
                if self[i] > key {
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
    fn test_clrs_insertion_sort(mut container: Container<i32>) -> bool {
        let mut data = container.data.clone();
        data.sort();
        container.clrs_insertion();
        Container { data } == container
    }

    #[quickcheck]
    fn insertion_sort(mut container: Container<i32>) -> bool {
        let mut data = container.data.clone();
        data.sort();
        container.insertion();
        Container { data } == container
    }
}
