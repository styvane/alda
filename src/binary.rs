//! Binary numbers operations.
//!
//! This module contains several utilities functions for
//! working with numbers represented as binary.

/// Add two n-bits binary integers, stored in two n-element
/// arrays. The arrays must be of the same length.
///
/// # Panics
/// This function will panic if the two slices are of different length.
///
/// # Examples
///
/// ```
/// use alda::binary;
///
/// let a = [1, 1, 0, 1, 0];
/// let b = [0, 0, 1, 1, 0];
/// let c = binary::add(&a, &b);
/// assert_eq!(vec![1, 1, 1, 0, 1], c);
/// ```
///
pub fn add(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert!(a.len() == b.len(), "Arrays must be of the same length");

    let mut c: Vec<u8> = Vec::new();
    let mut carry = 0;
    for (i, j) in a.iter().zip(b.iter()) {
        c.push((carry + i + j) % 2);
        carry = (carry + i + j) / 2;
    }

    if carry > 0 {
        c.push(carry);
    }

    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_array_of_same_len() {
        let a = [1, 0, 1, 0];
        let b = [0, 0, 1, 0];
        let c = add(&a, &b);
        assert_eq!(c, vec![1, 0, 0, 1]);
    }

    #[test]
    #[should_panic(expected = "Arrays must be of the same length")]
    fn test_add_array_of_diff_len() {
        let a = [1, 0, 1, 0, 0];
        let b = [0, 0, 1, 0];
        add(&a, &b);
    }
}
