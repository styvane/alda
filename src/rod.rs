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
        let mut cache = HashMap::new();
        cache.insert(0, 0);

        for index in 1..=size {
            let mut max = 0;

            for (ix, value) in self
                .prices
                .iter()
                .enumerate()
                .skip(1)
                .take_while(|&(i, _)| i <= size)
            {
                max = cmp::max(max, value + cache[&(index - ix)]);
            }
            cache.insert(index, max);
        }
        cache[&size]
    }

    /// List the pieces sizes that led to the maximum revenue.
    pub fn list_size(&self, size: usize) -> Vec<usize> {
        let mut sizes = HashMap::new();
        let mut cache = HashMap::new();
        cache.insert(0, 0);
        for index in 1..=size {
            let mut max = 0;
            for (jx, value) in self
                .prices
                .iter()
                .enumerate()
                .skip(1)
                .take_while(|&(i, _)| i <= index)
            {
                if max < value + cache[&(index - jx)] {
                    max = value + cache[&(index - jx)];
                    sizes.insert(index, jx);
                }
            }
            cache.insert(index, max);
        }

        let mut pieces = vec![];
        let mut n = size;

        while n > 0 {
            pieces.push(sizes[&n]);
            n -= sizes[&n];
        }

        pieces
    }

    /// Find the maximum for cutting a rod an selling it pieces
    /// with an additional cost for each cut.
    pub fn maximum_with_cut_cost(&self, size: usize, cost: usize) -> usize {
        let mut cache = HashMap::new();
        cache.insert(0, 0);

        for index in 1..=size {
            let mut max = 0;
            for (ix, value) in self
                .prices
                .iter()
                .enumerate()
                .skip(1)
                .take_while(|&(ix, _)| ix <= size)
            {
                max = cmp::max(max, value + cache[&(index - ix)] - cost);
            }
            cache.insert(index, max);
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

    #[test]
    fn test_list_size() {
        let v = vec![0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
        let rod = Rod::new(&v);

        assert_eq!(rod.list_size(1), [1]);
        assert_eq!(rod.list_size(2), [2]);
        assert_eq!(rod.list_size(3), [3]);
        assert_eq!(rod.list_size(4), [2, 2]);
        assert_eq!(rod.list_size(5), [2, 3]);
        assert_eq!(rod.list_size(6), [6]);
        assert_eq!(rod.list_size(7), [1, 6]);
        assert_eq!(rod.list_size(8), [2, 6]);
        assert_eq!(rod.list_size(9), [3, 6]);
        assert_eq!(rod.list_size(10), [10]);
    }
}
