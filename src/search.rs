//! This module contains utilities functions for searching
//! an item in a container.

use std::cmp;

/// Linear search an item in a container.
///
/// # Examples
///
/// ```
/// use alda::search;
///
/// let names = ["this", "is", "it"];
/// let found = search::linear(&names, &"is");
/// assert_eq!(found, Some(1));
/// ```
///
pub fn linear<T>(container: &[T], item: &T) -> Option<usize>
where
    T: cmp::Eq,
{
    for (i, v) in container.iter().enumerate() {
        if v == item {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
use quickcheck_macros::quickcheck;

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_linear_found_item(xs: Vec<isize>) -> bool {
        if xs.len() > 0 {
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
        assert!(linear(&s, item).is_none())
    }
}
