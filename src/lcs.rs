//! This module implements some longest common subsequence algorithms.
//!
//! The implementation of the various algorithms assumes the strings
//! contains only ASCII characters.

use std::cmp;

/// The type `LongSubSequence` contains the reference to two sequences
/// and their longest common subsequence
pub struct LongSubSequence<'a, 'b: 'a> {
    first: &'a str,
    second: &'b str,
    result: Option<String>,
}

impl<'a, 'b: 'a> LongSubSequence<'a, 'b> {
    /// Create a new instance of LongSubSequence.
    pub fn new(first: &'a str, second: &'b str) -> Self {
        LongSubSequence {
            first,
            second,
            result: None,
        }
    }

    /// Return the length of the LCS in a n-dimensional slice
    /// of which the the entries in row-major order.
    /// It also return a helper slice to help reconstruct the optimal solution.
    pub fn length_with_aux(&self) -> Option<(Vec<Box<Vec<usize>>>, Vec<Box<Vec<char>>>)> {
        if self.first.is_empty() || self.second.is_empty() {
            return None;
        }

        let mut len_vec = Vec::with_capacity(self.first.len() + 1);

        for _ in 0..self.first.len() + 1 {
            len_vec.push(Box::new(vec![0; self.second.len() + 1]));
        }

        let mut helper_vec = Vec::with_capacity(self.first.len() + 1);

        for _ in 0..self.first.len() + 1 {
            helper_vec.push(Box::new(vec![' '; self.second.len() + 1]));
        }

        let first: Vec<char> = self.first.chars().collect();
        let second: Vec<char> = self.second.chars().collect();

        let mut has_lcs = false;

        for i in 1..self.first.len() + 1 {
            for j in 1..self.second.len() + 1 {
                if first[i - 1] == second[j - 1] {
                    len_vec[i][j] = len_vec[i - 1][j - 1] + 1;
                    helper_vec[i][j] = '|';
                    has_lcs = true;
                } else if len_vec[i - 1][j] >= len_vec[i][j - 1] {
                    len_vec[i][j] = len_vec[i - 1][j];
                    helper_vec[i][j] = '>';
                } else {
                    len_vec[i][j] = len_vec[i][j - 1];
                    helper_vec[i][j] = '<';
                }
            }
        }

        if has_lcs {
            Some((len_vec, helper_vec))
        } else {
            None
        }
    }

    /// Return the length of the LCS in a n-dimensional slice
    /// of which the the entries in row-major order.
    /// using memoization.
    pub fn memoize_length(&self) -> Option<Vec<Box<Vec<isize>>>> {
        if self.first.is_empty() || self.second.is_empty() {
            return None;
        }

        let mut len_vec = Vec::with_capacity(self.first.len() + 1);

        for _ in 0..self.first.len() + 1 {
            len_vec.push(Box::new(vec![-1; self.second.len() + 1]));
        }

        let first: Vec<char> = self.first.chars().collect();
        let second: Vec<char> = self.second.chars().collect();

        let mut has_lcs = false;
        fn recursion<'a>(
            len_vec: &mut [Box<Vec<isize>>],
            first: &[char],
            second: &[char],
            ix: usize,
            jx: usize,
            has_lcs: &mut bool,
        ) -> isize {
            if len_vec[ix][jx] > -1 {
                return len_vec[ix][jx];
            }

            if ix == 0 || jx == 0 {
                len_vec[ix][jx] = 0;
                return len_vec[ix][jx];
            }
            if first[ix - 1] == second[jx - 1] {
                *has_lcs = true;
                len_vec[ix][jx] = recursion(len_vec, first, second, ix - 1, jx - 1, has_lcs) + 1;
                return len_vec[ix][jx];
            } else {
                len_vec[ix][jx] = cmp::max(
                    recursion(len_vec, first, second, ix - 1, jx, has_lcs),
                    recursion(len_vec, first, second, ix, jx - 1, has_lcs),
                );
                return len_vec[ix][jx];
            }
        }

        recursion(
            &mut len_vec,
            &first,
            &second,
            first.len(),
            second.len(),
            &mut has_lcs,
        );
        if has_lcs {
            Some(len_vec)
        } else {
            None
        }
    }

    /// Find the longest subsequence value.
    pub fn substr(&mut self) {
        fn solution<'a>(
            help_vec: &[Box<Vec<char>>],
            first: &Vec<char>,
            ix: usize,
            jx: usize,
            result: &mut Vec<char>,
        ) {
            if ix == 0 || jx == 0 {
                return;
            }

            if help_vec[ix][jx] == '|' {
                solution(help_vec, first, ix - 1, jx - 1, result);
                result.push(first[ix - 1]);
            } else if help_vec[ix][jx] == '>' {
                solution(help_vec, first, ix - 1, jx, result);
            } else {
                solution(help_vec, first, ix, jx - 1, result);
            }
        }

        if let Some((_, help_vec)) = self.length_with_aux() {
            let mut result = Vec::new();
            let first: Vec<char> = self.first.chars().collect();
            let (ix, jx) = (self.first.len(), self.second.len());
            solution(&help_vec, &first, ix, jx, &mut result);
            self.result = Some(result.into_iter().collect::<String>());
        }
    }

    /// Find the longest subsequence value without the helper table
    pub fn substr_without_aux(&mut self) {
        fn solution<'a>(
            len_vec: &[Box<Vec<usize>>],
            first: &Vec<char>,
            second: &Vec<char>,
            ix: usize,
            jx: usize,
            result: &mut Vec<char>,
        ) {
            if len_vec[ix][jx] == 0 {
                return;
            }

            if first[ix - 1] == second[jx - 1] {
                solution(len_vec, first, second, ix - 1, jx - 1, result);
                result.push(first[ix - 1]);
            } else if len_vec[ix - 1][jx] > len_vec[ix][jx - 1] {
                solution(len_vec, first, second, ix - 1, jx, result);
            } else {
                solution(len_vec, first, second, ix, jx - 1, result);
            }
        }

        if let Some((len_vec, _)) = self.length_with_aux() {
            let mut result = Vec::new();
            let first: Vec<char> = self.first.chars().collect();
            let second: Vec<char> = self.second.chars().collect();
            let (ix, jx) = (self.first.len(), self.second.len());
            solution(&len_vec, &first, &second, ix, jx, &mut result);
            self.result = Some(result.into_iter().collect::<String>());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_common_lcs() {
        let s1 = String::from("ACCGGTCGAGTGCGCGGAAGCCGGCCGAA");
        let s2 = String::from("GTCGTTCGGAATGCCGTTGCTCTGTAAA");
        let mut lcs = LongSubSequence::new(&s1, &s2);
        lcs.substr();
        assert_eq!(lcs.result.unwrap(), "GTCGTCGGAAGCCGGCCGAA");
    }

    #[test]
    fn test_one_empty_lcs() {
        let s1 = String::from("ACCGGTCGAGTGCGCGGAAGCCGGCCGAA");
        let s2 = String::new();
        let mut lcs = LongSubSequence::new(&s1, &s2);
        lcs.substr();
        assert_eq!(lcs.result, None);
    }

    #[test]
    fn test_no_lcs() {
        let s1 = String::from("ACCGGTCGAGTGCGCGGAAGCCGGCCGAA");
        let s2 = String::from("ZKZKHPRSUVWHKQSZXOPLMNXYW");
        let mut lcs = LongSubSequence::new(&s1, &s2);
        lcs.substr();
        assert_eq!(lcs.result, None);
    }

    #[test]
    fn test_empty_string() {
        let s1 = String::new();
        let s2 = String::new();
        let mut lcs = LongSubSequence::new(&s1, &s2);
        lcs.substr();
        assert_eq!(lcs.result, None);
    }

    #[test]
    fn test_without_aux_common_lcs() {
        let s1 = String::from("ACCGGTCGAGTGCGCGGAAGCCGGCCGAA");
        let s2 = String::from("GTCGTTCGGAATGCCGTTGCTCTGTAAA");
        let mut lcs = LongSubSequence::new(&s1, &s2);
        lcs.substr_without_aux();
        assert_eq!(lcs.result.unwrap(), "GTCGTCGGAAGCCGGCCGAA");
    }

    #[test]
    fn test_memoize_length() {
        let s1 = String::from("ACCGGTCGAGTGCGCGGAAGCCGGCCGAA");
        let s2 = String::from("GTCGTTCGGAATGCCGTTGCTCTGTAAA");
        let lcs = LongSubSequence::new(&s1, &s2);
        let c = lcs.memoize_length().unwrap();
        assert_eq!(c[s1.len()][s2.len()], 20);
    }
}
