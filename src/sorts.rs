//! Sorting routines.
//!
//! This module contains several functions for sorting container elements
//!

use std::cmp::Ord;
use std::ops::{Deref, DerefMut};

pub struct Container<'a, T: Ord + Clone>(&'a mut [T]);

impl<'a, T: Ord + Clone> Deref for Container<'a, T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Ord + Clone> DerefMut for Container<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T: Ord + Clone> Container<'a, T> {
    pub fn new(data: &'a mut [T]) -> Container<'a, T> {
        Container(data)
    }

    /// Sort container elements using CLRS insertion sort algorithm.
    ///
    /// # Examples
    ///
    /// ```
    /// use alda::sorts::Container;
    ///
    /// let mut v = vec![3, 11, 0, 7, -1, 9, 31];
    /// let mut c = Container::new(&mut v);
    /// c.clrs_insertion_sort();
    /// assert_eq!(v, vec![-1, 0, 3, 7, 9, 11, 31]);
    /// ```
    ///
    pub fn clrs_insertion_sort(&mut self) {
        for i in 1..self.len() {
            let key = self[i].clone();
            let mut j = i - 1;
            while self[j] > key {
                self.swap(j, j + 1);
                if j == 0 {
                    break;
                }
                j = j - 1;
            }
        }
    }

    /// Sort container elements without cloning or copying
    /// using insertion sort algorithm.
    ///
    /// # Examples
    ///
    /// ```
    /// use alda::sorts::Container;
    ///
    /// let mut v = vec![3, 0, 7, -1, 31];
    /// let mut c = Container::new(&mut v);
    /// c.insertion_sort();
    /// assert_eq!(v, vec![-1, 0, 3, 7, 31]);
    /// ```
    ///
    pub fn insertion_sort(&mut self) {
        for i in 1..self.len() {
            for j in (0..i).rev() {
                if self[j] > self[j + 1] {
                    self.swap(j + 1, j);
                } else {
                    break;
                }
            }
        }
    }

    /// Sort container elements in nonincreasing order using
    /// insertion sort.
    ///
    /// # Examples
    ///
    /// ```
    /// use alda::sorts::Container;
    ///
    /// let mut v = vec![0, 7, -1, 91, 12];
    /// let mut c = Container::new(&mut v);
    /// c.reverse_insertion_sort();
    /// assert_eq!(v, vec![91, 12, 7 , 0, -1]);
    /// ```
    ///
    pub fn reverse_insertion_sort(&mut self) {
        for i in 1..self.len() {
            for j in (0..i).rev() {
                if self[j] < self[j + 1] {
                    self.swap(j, j + 1);
                } else {
                    break;
                }
            }
        }
    }

    /// Sort container elements using selection sort
    ///
    /// # Examples
    ///
    /// ```
    /// use alda::sorts::Container;
    ///
    /// let mut v  = vec![0, -2, -1];
    /// let mut c = Container::new(&mut v);
    /// c.selection_sort();
    /// assert_eq!(v, vec![-2, -1, 0]);
    /// ```
    ///
    pub fn selection_sort(&mut self) {
        if self.len() == 0 {
            return;
        }
        for i in 0..(self.len() - 1) {
            let mut min = i;
            for (j, n) in self.iter().enumerate().skip(i + 1) {
                if n < &self[min] {
                    min = j;
                }
            }
            self.swap(min, i);
        }
    }

    /// Sort container elements using merge sort.
    ///
    /// # Examples
    ///
    /// ```
    /// use alda::sorts::Container;
    ///
    /// let mut v = vec![0, -1, 4, -6];
    /// let mut c = Container::new(&mut v);
    /// let n = c.len();
    /// c.mergesort(0, n);
    /// assert_eq!(v, vec![-6, -1, 0, 4]);
    /// ```
    ///
    pub fn mergesort(&mut self, start: usize, end: usize) {
        if start < end - 1 {
            let mid = (start + end) / 2;
            self.mergesort(start, mid);
            self.mergesort(mid, end);
            self.merge(start, end);
        }
    }

    // Merge two sorted containers.
    #[allow(dead_code)]
    fn clrs_merge(&mut self, start: usize, mid: usize, end: usize) {
        let n1 = mid - start;

        let mut left = Vec::with_capacity(n1);
        for i in 0..n1 {
            left.push(self[start + i].clone());
        }

        let n2 = end - mid;

        let mut right = Vec::with_capacity(n2);
        for i in 0..n2 {
            right.push(self[mid + i].clone());
        }

        let mut i = 0;
        let mut j = 0;

        for k in start..end {
            if i < n1 && j < n2 && left[i] <= right[j] || i < n1 && j == n2 {
                self[k] = left[i].clone();
                i = i + 1;
            } else {
                self[k] = right[j].clone();
                j = j + 1;
            }
        }
    }

    // Merge two sorted containers.
    fn merge(&mut self, start: usize, end: usize) {
        let c = self[start..end].iter().cloned().collect::<Vec<_>>();
        let len = c.len();

        let (left, right) = c.split_at(len / 2);

        let mut left = left.iter().peekable();
        let mut right = right.iter().peekable();

        for i in start..end {
            if let (Some(l), Some(r)) = (left.peek(), right.peek()) {
                if l <= r {
                    self[i] = left.next().unwrap().clone();
                } else {
                    self[i] = right.next().unwrap().clone();
                }
            } else if left.peek().is_some() {
                self[i] = left.next().unwrap().clone();
            } else {
                self[i] = right.next().unwrap().clone();
            }
        }
    }

    /// Recursive sort container using insertion sort
    ///
    /// # Examples
    ///
    /// ```
    /// use alda::sorts::Container;
    ///
    /// let mut a = vec![-9, 0, 7, -1, 4];
    /// let n = a.len();
    /// let mut c = Container::new(&mut a);
    /// c.recursive_insort(n);
    /// assert_eq!(a, vec![-9, -1, 0, 4, 7]);
    /// ```
    ///
    pub fn recursive_insort(&mut self, n: usize) {
        if n <= 1 {
            return;
        }
        self.recursive_insort(n - 1);
        let last = self[n - 1].clone();
        for j in (0..(n - 1)).rev() {
            if self[j] > last {
                self.swap(j + 1, j);
            }
        }
    }
}
#[cfg(test)]
use quickcheck_macros::quickcheck;

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_clrs_insertion_sort(mut xs: Vec<isize>) -> bool {
        let mut want = xs.clone();
        let mut c = Container(&mut xs);
        c.clrs_insertion_sort();

        want.sort();
        c.0 == want
    }

    #[test]
    fn test_clrs_insertion_sort_empty() {
        let mut c: Container<&[i32]> = Container::new(&mut []);
        c.clrs_insertion_sort();
        assert!(c.is_empty())
    }

    #[quickcheck]
    fn test_insertion_sort(mut xs: Vec<isize>) -> bool {
        let mut want = xs.clone();
        want.sort();
        let mut c = Container(&mut xs);
        c.insertion_sort();
        c.0 == want
    }

    #[quickcheck]
    fn test_reverse_insertion_sort(mut xs: Vec<isize>) -> bool {
        let mut want = xs.clone();
        want.sort();
        want.reverse();
        let mut c = Container(&mut xs);
        c.reverse_insertion_sort();
        c.0 == want
    }

    #[quickcheck]
    fn test_selection_sort(mut xs: Vec<isize>) -> bool {
        let mut want = xs.clone();
        let mut c = Container(&mut xs);
        want.sort();
        c.selection_sort();
        c.0 == want
    }

    #[test]
    fn test_merge() {
        let mut xs = vec![5, -1, 3, 0, 4, 2];
        let mut c = Container(&mut xs);
        c.clrs_merge(1, 3, 4);
        assert_eq!(c.0, [5, -1, 0, 3, 4, 2]);
    }

    #[quickcheck]
    fn test_mergesort(mut xs: Vec<isize>) -> bool {
        if xs.is_empty() {
            return true;
        }
        let mut want = xs.clone();
        want.sort();

        let mut c = Container(&mut xs);
        let n = c.len();
        c.mergesort(0, n);
        c.0 == want
    }

    #[quickcheck]
    fn test_recursive_insort(mut xs: Vec<isize>) -> bool {
        if xs.is_empty() {
            return true;
        }
        let mut want = xs.clone();
        let mut c = Container(&mut xs);
        want.sort();
        let n = c.len();
        c.recursive_insort(n);
        c.0 == want
    }
}
