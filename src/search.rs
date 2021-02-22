//! This module contains utilities functions for searching
//! an item in a container.

use num_traits::{Num, NumOps};
use std::cmp::{self, Ordering};

/// Linear search for a key in a container.
///
/// # Examples
///
/// ```
/// use alda::search;
///
/// let names = ["this", "is", "it"];
/// assert_eq!(search::linear(&names, &"is"), Some(1));
/// ```
///
pub fn linear<T>(container: &[T], key: &T) -> Option<usize>
where
    T: cmp::Eq,
{
    for (i, v) in container.iter().enumerate() {
        if v == key {
            return Some(i);
        }
    }
    None
}

/// Search for a key in a container using binary search.
///
/// # Examples
///
/// ```
/// use alda::search;
///
/// let c = [1, -1, 9, 0];
/// assert_eq!(search::binsearch(&c, &9), Some(2));
///
/// assert_eq!(None, search::binsearch(&c, &2));
/// ```
///
pub fn binsearch<T>(container: &[T], key: &T) -> Option<usize>
where
    T: cmp::Ord,
{
    let mut lower = 0;
    let mut upper = container.len();
    while lower < upper {
        let mid = (lower + upper) / 2;
        match container[mid].cmp(key) {
            Ordering::Equal => {
                return Some(mid);
            }
            Ordering::Less => {
                lower = mid + 1;
            }
            Ordering::Greater => {
                upper = mid - 1;
            }
        }
    }
    None
}

/// Search for a key in a collection using  recursive binary search.
///
/// # Examples
///
///```
/// use alda::search;
///
/// let a = [-9, -1, 0, 7];
/// assert_eq!(search::recursive_binsearch(&a, &8), None);
/// assert_eq!(search::recursive_binsearch(&a, &0), Some(2));
/// ```
///
pub fn recursive_binsearch<T>(container: &[T], key: &T) -> Option<usize>
where
    T: cmp::Ord,
{
    if container.is_empty() {
        return None;
    }

    let mid = container.len() / 2;
    match key.cmp(&container[mid]) {
        Ordering::Equal => Some(mid),
        Ordering::Less => recursive_binsearch(&container[..mid], key),
        Ordering::Greater => recursive_binsearch(&container[mid + 1..], key).map(|x| {
            if mid % 2 == 0 {
                x + mid + 1
            } else {
                x + mid
            }
        }),
    }
}

// Find maximum crossing subarray.
#[allow(dead_code)]
fn find_maximum_cross_subarray<T>(
    arr: &[T],
    lower: usize,
    mid: usize,
    upper: usize,
) -> (usize, usize, T)
where
    T: cmp::Ord + Copy + Num + NumOps,
{
    let mut max_left = mid - 1;
    let mut left_sum = arr[mid - 1];
    let mut sum = arr[mid - 1];

    for i in (lower..mid).rev() {
        if i < mid - 1 {
            sum = sum + arr[i];
            if left_sum < sum {
                left_sum = sum;
                max_left = i;
            }
        }
    }

    let mut max_right = mid;
    let mut sum = arr[mid];
    let mut right_sum = arr[mid];
    for i in mid..upper {
        if i > mid {
            sum = sum + arr[i];
            if right_sum < sum {
                right_sum = sum;
                max_right = i;
            }
        }
    }

    (max_left, max_right, left_sum + right_sum)
}

/// Find a maximum subarray of container using recursion
///
/// # Examples
///
/// ```
/// use alda::search;
///
/// let a = &[-99, -1, 2, 9, -11,  -3, 4, 89, -2];
/// assert_eq!(search::recursive_find_maximum_subarray(a, 0, a.len()), (6, 7, 93))
/// ```
///
pub fn recursive_find_maximum_subarray<T>(
    container: &[T],
    lower: usize,
    upper: usize,
) -> (usize, usize, T)
where
    T: cmp::Ord + Copy + Num + NumOps,
{
    assert!(!container.is_empty());

    if lower == upper - 1 {
        return (lower, lower, container[lower]);
    }
    let mid = (lower + upper) / 2;
    let (left_lower, left_upper, left_sum) = recursive_find_maximum_subarray(container, lower, mid);
    let (right_lower, right_upper, right_sum) =
        recursive_find_maximum_subarray(container, mid, upper);
    let (cross_lower, cross_upper, cross_sum) =
        find_maximum_cross_subarray(container, lower, mid, upper);
    if left_sum >= right_sum && left_sum > cross_sum {
        (left_lower, left_upper, left_sum)
    } else if right_sum >= left_sum && right_sum >= cross_sum {
        (right_lower, right_upper, right_sum)
    } else {
        (cross_lower, cross_upper, cross_sum)
    }
}

#[cfg(test)]
use quickcheck_macros::quickcheck;

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_linear_found_item(xs: Vec<isize>) -> bool {
        if !xs.is_empty() {
            let item = &xs[0];
            linear(&xs, item).is_some()
        } else {
            true
        }
    }

    #[test]
    fn test_linear_search_str() {
        let s = ["foo", "bar"];
        let item = &"spam";
        assert_eq!(linear(&s, item), None)
    }

    #[test]
    fn test_binsearch_str() {
        let s = ["foo", "bar", "foobar"];
        let item = &"foobar";
        assert_eq!(linear(&s, item), Some(2))
    }

    #[quickcheck]
    fn test_recursive_search_number(xs: Vec<isize>) -> bool {
        if xs.is_empty() {
            return true;
        }

        let ix = xs.len() / 2;
        let key = &xs[ix];
        let found = recursive_binsearch(&xs, key);
        found == Some(ix)
    }

    #[test]
    fn test_maximum_cross_array() {
        let a = &[9, -1, 2, 9, -11, -3, 4, 9, -2];
        assert_eq!(
            find_maximum_cross_subarray(a, 0, a.len() / 2, a.len()),
            (0, 7, 18)
        )
    }

    #[test]
    fn test_recursive_find_maximum_subarray() {
        let a = &[9, -1, 2, 9, -11, -3, 4, 9, -2];
        assert_eq!(recursive_find_maximum_subarray(a, 0, 6), (0, 3, 19))
    }
}
