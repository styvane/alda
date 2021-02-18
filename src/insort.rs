//! An insertion sort routines.
//!
//! This module contains several functions for sorting container elements
//! using insertion sort.
//!

use std::cmp;

/// Sort container elements using CLRS insertion sort algorithm.
///
/// # Examples
///
/// ```
/// use alda::insort;
///
/// let mut c = [3, 11, 0, 7, -1, 9, 31];
/// insort::clrs_sort(&mut c);
/// assert_eq!(c, [-1, 0, 3, 7, 9, 11, 31]);
/// ```
///
pub fn clrs_sort<T>(container: &mut [T])
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

/// Sort container elements without cloning or copying.
///
/// # Examples
///
/// ```
/// use alda::insort;
///
/// let mut c = [3, 0, 7, -1, 31];
/// insort::sort(&mut c);
/// assert_eq!(c, [-1, 0, 3, 7, 31]);
/// ```
///
pub fn sort<T>(container: &mut [T])
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

/// Sort container elements in nonincreasing order.
///
/// # Examples
///
/// ```
/// use alda::insort;
///
/// let mut c = [0, 7, -1, 91, 12];
/// insort::reverse_sort(&mut c);
/// assert_eq!(c, [91, 12, 7 , 0, -1]);
/// ```
///
pub fn reverse_sort<T>(container: &mut [T])
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

#[cfg(test)]
use quickcheck_macros::quickcheck;

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_clrs_sort(xs: Vec<i32>) -> bool {
        let mut xs = xs;
        clrs_sort(&mut xs);
        let mut want = xs.clone();
        want.sort();
        xs == want
    }

    #[test]
    fn test_clrs_sort_empty() {
        let mut list: [i32; 0] = [];
        clrs_sort(&mut list);
        assert_eq!(list, [])
    }

    #[quickcheck]
    fn test_sort(xs: Vec<isize>) -> bool {
        let mut xs = xs;
        let mut want = xs.clone();
        want.sort();
        sort(&mut xs);
        xs == want
    }

    #[test]
    fn test_sort_empty() {
        let mut list: [i32; 0] = [];
        sort(&mut list);
        assert_eq!(list, [])
    }

    #[quickcheck]
    fn test_reverse_sort(xs: Vec<isize>) -> bool {
        let mut xs = xs;
        let mut want = xs.clone();
        want.sort();
        want.reverse();
        reverse_sort(&mut xs);
        xs == want
    }
}
