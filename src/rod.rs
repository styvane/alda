//! Various rod cutting implementation
//!
//! This module contains a various implementation of rod cutting algorithms.

use std::cmp;

/// The type `Rod` contains the data for computing the maximum revenue
/// for cutting a rod and selling pieces.
pub struct Rod<'a> {
    prices: &'a Vec<usize>,
}

impl<'a> Rod<'a> {
    /// Create a new rode with associated chunk prices
    pub fn new(prices: &'a Vec<usize>) -> Rod {
        Rod { prices }
    }

    /// Recursively find the maximum revenue for cutting a rod and selling
    /// it pieces.
    pub fn recursive_maximum(&self, n: usize) -> usize {
        fn compute_maximum(prices: &Vec<usize>, n: usize) -> usize {
            let mut p = 0;

            for (i, value) in prices
                .iter()
                .enumerate()
                .skip(1)
                .take_while(|(i, _)| i <= &n)
            {
                p = cmp::max(p, value + compute_maximum(prices, n - i))
            }
            p
        }

        compute_maximum(self.prices, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_maximum() {
        let v = vec![0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
        let rod = Rod::new(&v);

        assert_eq!(rod.recursive_maximum(1), 1);
        assert_eq!(rod.recursive_maximum(2), 5);
        assert_eq!(rod.recursive_maximum(3), 8);
        assert_eq!(rod.recursive_maximum(4), 10);
        assert_eq!(rod.recursive_maximum(5), 13);
        assert_eq!(rod.recursive_maximum(6), 17);
        assert_eq!(rod.recursive_maximum(7), 18);
        assert_eq!(rod.recursive_maximum(8), 22);
        assert_eq!(rod.recursive_maximum(9), 25);
        assert_eq!(rod.recursive_maximum(10), 30);
    }

    #[test]
    fn test_recursive_maximum_empty() {
        let v = vec![];
        let rod = Rod::new(&v);

        assert_eq!(rod.recursive_maximum(1), 0);
    }
}
