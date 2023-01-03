//! Sorting algorithms
//!
//! This module implements various sorting algorithms.

use rand::Rng;

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
    fn naive_insertion_sort(&mut self, compare: impl Fn(&T, &T) -> bool);

    /// Alternative version of CLRS insertion algorithm.
    fn insertion_sort(&mut self, compare: impl Fn(&T, &T) -> bool);

    /// Selection sort algorithm.
    fn selection_sort(&mut self, compare: impl Fn(&T, &T) -> bool);

    /// Merge sort algorithm.
    fn merge_sort(&mut self, start: usize, end: usize);

    /// Recursive insertion sort.

    /// Recursively sort the N - 1 elements in the container
    /// and the insert the N-th element in the sorted container.
    fn rec_insertion_sort(&mut self);

    /// QuickSort algorithm.
    fn quick_sort(&mut self, start: usize, end: usize);

    /// QuickSort algorithm.
    fn randomize_quick_sort(&mut self, start: usize, end: usize);
}

impl<T> Sort<T> for Container<T>
where
    T: Eq + PartialEq + PartialOrd + Ord + Clone,
{
    fn naive_insertion_sort(&mut self, compare: impl Fn(&T, &T) -> bool) {
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

    fn insertion_sort(&mut self, compare: impl Fn(&T, &T) -> bool) {
        if self.len() <= 1 {
            return;
        }
        for j in 1..self.len() {
            let key = self[j].clone();
            for i in (0..j).rev() {
                if compare(&self[i], &key) {
                    self.swap(i + 1, i);
                }
            }
        }
    }

    fn selection_sort(&mut self, compare: impl Fn(&T, &T) -> bool) {
        if self.len() <= 1 {
            return;
        }
        for j in 0..self.len() - 1 {
            let mut index = j;
            for i in (j + 1)..self.len() {
                if compare(&self[i], &self[index]) {
                    index = i;
                }
            }
            self.swap(index, j);
        }
    }

    fn merge_sort(&mut self, start: usize, end: usize) {
        if end > 0 && start < end - 1 {
            let middle = (end + start) / 2;
            self.merge_sort(start, middle);
            self.merge_sort(middle, end);
            self.merge(start, middle, end);
        }
    }

    fn rec_insertion_sort(&mut self) {
        fn insort<T: Clone + PartialOrd>(data: &mut [T]) {
            if data.len() <= 1 {
                return;
            }
            let len = data.len() - 1;
            let key = data[len].clone();

            insort(&mut data[0..len]);
            for i in (0..len).rev() {
                if data[i] > key {
                    data.swap(i + 1, i);
                }
            }
        }
        insort(&mut self.data);
    }

    fn quick_sort(&mut self, start: usize, end: usize) {
        if start < end {
            let mid = self.partition(start, end);
            self.quick_sort(start, mid);
            self.quick_sort(mid + 1, end);
        }
    }

    fn randomize_quick_sort(&mut self, start: usize, end: usize) {
        let index = rand::thread_rng().gen_range(start..end);
        self.swap(index, end - 1);
        if start < end {
            let mid = self.partition(start, end);
            self.quick_sort(start, mid);
            self.quick_sort(mid + 1, end);
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
    fn test_insertion_sort_ascending(mut container: Container<i32>) -> bool {
        let mut data = container.data.clone();
        data.sort();
        container.naive_insertion_sort(|a, b| a > b);
        Container { data } == container
    }

    #[quickcheck]
    fn test_naive_insertion_sort_descending(mut container: Container<i32>) -> bool {
        let mut data = container.data.clone();
        data.sort_by(|a, b| b.cmp(a));
        container.naive_insertion_sort(|a, b| a < b);
        Container { data } == container
    }

    #[quickcheck]
    fn insertion_sort_ascending(mut container: Container<i32>) -> bool {
        let mut data = container.data.clone();
        data.sort();
        container.insertion_sort(|a, b| a > b);
        Container { data } == container
    }

    #[quickcheck]
    fn insertion_sort_descending(mut container: Container<i32>) -> bool {
        let mut data = container.data.clone();
        data.sort_by(|a, b| b.cmp(a));
        container.insertion_sort(|a, b| a < b);
        Container { data } == container
    }

    #[quickcheck]
    fn selection_sort_ascending(mut container: Container<i32>) -> bool {
        let mut data = container.data.clone();
        data.sort();
        container.selection_sort(|a, b| a < b);
        Container { data } == container
    }

    #[quickcheck]
    fn selection_sort_descending(mut container: Container<i32>) -> bool {
        let mut data = container.data.clone();
        data.sort_by(|a, b| b.cmp(a));
        container.selection_sort(|a, b| a > b);
        Container { data } == container
    }

    #[quickcheck]
    fn merge_sort_ascending(mut container: Container<i32>) -> bool {
        let mut data = container.data.clone();
        data.sort();
        container.merge_sort(0, data.len());
        Container { data } == container
    }

    #[test]
    fn rec_insertion_sort_ascending() {
        let mut container = Container::new(vec![-9, 0, 1, 3, 2]);
        let mut data = container.data.clone();
        data.sort();
        container.rec_insertion_sort();
        assert_eq!(Container { data }, container);
    }

    #[test]
    fn quicksort() {
        let mut container = Container::new(vec![-9, 0, 1, 3, 2]);
        let mut data = container.data.clone();
        data.sort();
        container.quick_sort(0, data.len());
        assert_eq!(Container { data }, container);
    }
}
