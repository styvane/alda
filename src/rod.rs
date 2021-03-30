//! Various rod cutting implementation
//!
//! This module contains a various implementation of rod cutting algorithms.

use std::cmp;
use std::collections::HashMap;

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
    pub fn recursive_maximum(&self, size: usize) -> usize {
        fn compute_maximum(prices: &Vec<usize>, size: usize) -> usize {
            let mut max = 0;

            for (i, value) in prices
                .iter()
                .enumerate()
                .skip(1)
                .take_while(|&(i, _)| i <= size)
            {
                max = cmp::max(max, value + compute_maximum(prices, size - i))
            }
            max
        }

        compute_maximum(self.prices, size)
    }

    /// Find the maximum revenue for cutting a rod and selling
    /// it pieces using a top-down approach with memoization.
    pub fn maximum_with_memoization(&self, size: usize) -> usize {
        let mut cache = HashMap::<usize, usize>::new();

        fn memoize_max(
            prices: &Vec<usize>,
            size: usize,
            cache: &mut HashMap<usize, usize>,
        ) -> usize {
            if cache.contains_key(&size) {
                return cache[&size];
            }

            let mut max = 0;
            for (i, value) in prices
                .iter()
                .enumerate()
                .skip(1)
                .take_while(|&(i, _)| i <= size)
            {
                max = cmp::max(max, value + memoize_max(prices, size - i, cache));
            }
            cache.insert(size, max);
            max
        }

        memoize_max(self.prices, size, &mut cache)
    }

    /// Find the maximum revenue for cutting a rod and selling
    /// it pieces using the bottom up approach.
    pub fn maximum_with_bottom_up(&self, size: usize) -> usize {
        let mut cache = HashMap::<usize, usize>::new();
        cache.insert(0, 0);
        for (ix, value) in self
            .prices
            .iter()
            .enumerate()
            .skip(1)
            .take_while(|&(i, _)| i <= size)
        {
            let mut max = 0;
            for j in 1..=ix {
                max = cmp::max(max, value + self.prices[ix - j]);
            }
            cache.insert(ix, max);
        }

        cache[&size]
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

    #[test]
    fn test_memoized_maximum() {
        let v = vec![0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
        let rod = Rod::new(&v);

        assert_eq!(rod.maximum_with_memoization(1), 1);
        assert_eq!(rod.maximum_with_memoization(2), 5);
        assert_eq!(rod.maximum_with_memoization(3), 8);
        assert_eq!(rod.maximum_with_memoization(4), 10);
        assert_eq!(rod.maximum_with_memoization(5), 13);
        assert_eq!(rod.maximum_with_memoization(6), 17);
        assert_eq!(rod.maximum_with_memoization(7), 18);
        assert_eq!(rod.maximum_with_memoization(8), 22);
        assert_eq!(rod.maximum_with_memoization(9), 25);
        assert_eq!(rod.maximum_with_memoization(10), 30);
    }
}
