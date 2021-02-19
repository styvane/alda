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
}
