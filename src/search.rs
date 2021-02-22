//! This module contains utilities functions for searching
//! an item in a container.

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
}
