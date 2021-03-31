//! This module contains the routine to calculate Fibonacci numbers.
//!

use std::collections::HashMap;

/// The `Fib` type contains a nth Fibonacci value.
pub struct Fib {
    pub rank: usize,
    pub value: Option<usize>,
}

impl Fib {
    /// Create a new Fibonacci type.
    pub fn new(rank: usize) -> Self {
        Fib { rank, value: None }
    }

    /// Compute the Fibonacci value.
    pub fn compute(&mut self) {
        fn fib(n: &usize) -> usize {
            let mut cache = HashMap::new();
            cache.insert(0, 0);
            cache.insert(1, 1);

            for i in 2..=*n {
                cache.insert(i, cache[&(i - 1)] + cache[&(i - 2)]);
            }
            cache[&n]
        }

        let value = fib(&self.rank);
        self.value = Some(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        let mut fib = Fib::new(5);
        fib.compute();
        assert_eq!(fib.value, Some(5));

        fib = Fib::new(6);
        fib.compute();
        assert_eq!(fib.value, Some(8));
    }
}
