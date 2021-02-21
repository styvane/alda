//! Sorting routines.
//!
//! This module contains several functions for sorting container elements
//!

use std::cmp;

/// Sort container elements using CLRS insertion sort algorithm.
///
/// # Examples
///
/// ```
/// use alda::sorts;
///
/// let mut c = [3, 11, 0, 7, -1, 9, 31];
/// sorts::clrs_insertion_sort(&mut c);
/// assert_eq!(c, [-1, 0, 3, 7, 9, 11, 31]);
/// ```
///
pub fn clrs_insertion_sort<T>(container: &mut [T])
where
    T: cmp::Ord + Clone,
{
    for i in 1..container.len() {
        let key = container[i].clone();
        let mut j = i - 1;
        while container[j] > key {
            container.swap(j, j + 1);
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
/// use alda::sorts;
///
/// let mut c = [3, 0, 7, -1, 31];
/// sorts::insertion_sort(&mut c);
/// assert_eq!(c, [-1, 0, 3, 7, 31]);
/// ```
///
pub fn insertion_sort<T>(container: &mut [T])
where
    T: cmp::Ord,
{
    for i in 1..container.len() {
        for j in (0..i).rev() {
            if container[j] > container[j + 1] {
                container.swap(j + 1, j);
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
/// use alda::sorts;
///
/// let mut c = [0, 7, -1, 91, 12];
/// sorts::reverse_insertion_sort(&mut c);
/// assert_eq!(c, [91, 12, 7 , 0, -1]);
/// ```
///
pub fn reverse_insertion_sort<T>(container: &mut [T])
where
    T: cmp::Ord,
{
    for i in 1..container.len() {
        for j in (0..i).rev() {
            if container[j] < container[j + 1] {
                container.swap(j, j + 1);
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
/// use alda::sorts;
///
/// let mut c  = [0, -2, -1];
/// sorts::selection_sort(&mut c);
/// assert_eq!(c, [-2, -1, 0]);
/// ```
///
pub fn selection_sort<T>(container: &mut [T])
where
    T: cmp::Ord,
{
    if container.len() == 0 {
        return;
    }
    for i in 0..(container.len() - 1) {
        let mut min = i;
        for (j, n) in container.iter().enumerate().skip(i + 1) {
            if n < &container[min] {
                min = j;
            }
        }
        container.swap(min, i);
    }
}

/// Sort container elements using merge sort.
///
/// # Examples
///
/// ```
/// use alda::sorts;
///
/// let mut c = [0, -1, 4, -6];
/// let n = c.len();
/// sorts::mergesort(&mut c, 0, n);
/// assert_eq!(c, [-6, -1, 0, 4]);
/// ```
///
pub fn mergesort<T>(container: &mut [T], start: usize, end: usize)
where
    T: cmp::Ord + Clone,
{
    if start < end - 1 {
        let mid = (start + end) / 2;
        mergesort(container, start, mid);
        mergesort(container, mid, end);
        merge(container, start, end);
    }
}

// Merge two sorted containers.
#[allow(dead_code)]
fn clrs_merge<T>(container: &mut [T], start: usize, mid: usize, end: usize)
where
    T: cmp::Ord + Clone,
{
    let n1 = mid - start;

    let mut left = Vec::with_capacity(n1);
    for i in 0..n1 {
        left.push(container[start + i].clone());
    }

    let n2 = end - mid;

    let mut right = Vec::with_capacity(n2);
    for i in 0..n2 {
        right.push(container[mid + i].clone());
    }

    let mut i = 0;
    let mut j = 0;

    for k in start..end {
        if i < n1 && j < n2 && left[i] <= right[j] || i < n1 && j == n2 {
            container[k] = left[i].clone();
            i = i + 1;
        } else {
            container[k] = right[j].clone();
            j = j + 1;
        }
    }
}

// Merge two sorted containers.
fn merge<T>(container: &mut [T], start: usize, end: usize)
where
    T: cmp::Ord + Clone,
{
    let c = container[start..end].iter().cloned().collect::<Vec<_>>();
    let len = c.len();

    let (left, right) = c.split_at(len / 2);

    let mut left = left.iter().peekable();
    let mut right = right.iter().peekable();

    for i in start..end {
        if let (Some(l), Some(r)) = (left.peek(), right.peek()) {
            if l <= r {
                container[i] = left.next().unwrap().clone();
            } else {
                container[i] = right.next().unwrap().clone();
            }
        } else if left.peek().is_some() {
            container[i] = left.next().unwrap().clone();
        } else {
            container[i] = right.next().unwrap().clone();
        }
    }
}

/// Recursive sort container using insertion sort
///
/// # Examples
///
/// ```
/// use alda::sorts;
///
/// let mut a = [-9, 0, 7, -1, 4];
/// let n = a.len();
/// sorts::recursive_insort(&mut a, n);
/// assert_eq!(a, [-9, -1, 0, 4, 7]);
/// ```
///
pub fn recursive_insort<T>(container: &mut [T], n: usize)
where
    T: cmp::Ord + Clone,
{
    if n <= 1 {
        return;
    }
    recursive_insort(container, n - 1);
    let last = container[n - 1].clone();
    for j in (0..(n - 1)).rev() {
        if container[j] > last {
            container.swap(j + 1, j);
        }
    }
}

#[cfg(test)]
use quickcheck_macros::quickcheck;

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_clrs_insertion_sort(xs: Vec<isize>) -> bool {
        let mut xs = xs;
        clrs_insertion_sort(&mut xs);
        let mut want = xs.clone();
        want.sort();
        xs == want
    }

    #[test]
    fn test_clrs_insertion_sort_empty() {
        let mut list: [i32; 0] = [];
        clrs_insertion_sort(&mut list);
        assert_eq!(list, [])
    }

    #[quickcheck]
    fn test_insertion_sort(xs: Vec<isize>) -> bool {
        let mut xs = xs;
        let mut want = xs.clone();
        want.sort();
        insertion_sort(&mut xs);
        xs == want
    }

    #[test]
    fn test_insertion_sort_empty() {
        let mut list: [i32; 0] = [];
        insertion_sort(&mut list);
        assert_eq!(list, [])
    }

    #[quickcheck]
    fn test_reverse_insertion_sort(xs: Vec<isize>) -> bool {
        let mut xs = xs;
        let mut want = xs.clone();
        want.sort();
        want.reverse();
        reverse_insertion_sort(&mut xs);
        xs == want
    }

    #[quickcheck]
    fn test_selection_sort(xs: Vec<isize>) -> bool {
        let mut xs = xs;
        let mut want = xs.clone();
        want.sort();
        selection_sort(&mut xs);
        xs == want
    }

    #[test]
    fn test_merge() {
        let mut a = [5, -1, 3, 0, 4, 2];
        clrs_merge(&mut a, 1, 3, 4);
        assert_eq!(a, [5, -1, 0, 3, 4, 2]);
    }

    #[quickcheck]
    fn test_mergesort(xs: Vec<isize>) -> bool {
        let mut xs = xs;
        let mut want = xs.clone();
        want.sort();
        if xs.len() > 0 {
            let n = xs.len();
            mergesort(&mut xs, 0, n);
            return xs == want;
        }
        true
    }

    #[quickcheck]
    fn test_recursive_insort(xs: Vec<isize>) -> bool {
        let mut xs = xs;
        let mut want = xs.clone();
        want.sort();
        if xs.len() > 0 {
            let n = xs.len();
            recursive_insort(&mut xs, n);
            return xs == want;
        }
        true
    }
}
