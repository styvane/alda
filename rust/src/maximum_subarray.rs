//! A solution to the maximum sub-array problem.

use crate::Container;

use std::cmp::Ordering;

/// The `MaxSubarray` type represents the maximum the maximum
/// sub-array data type.
#[derive(Debug, Clone, Default)]
pub struct MaxSubarray<T> {
    /// Index of the most left element in the subarray.
    pub left_index: usize,

    /// Index of the most right element in the subarray.
    pub right_index: usize,

    /// Sum of the element in the [left_index; right_index]
    pub sum: T,
}

impl<T> PartialEq for MaxSubarray<T>
where
    T: PartialEq + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.sum == other.sum
    }
}

impl<T> PartialOrd for MaxSubarray<T>
where
    T: PartialEq + Eq + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.sum.partial_cmp(&other.sum)
    }
}
impl<T> Eq for MaxSubarray<T> where T: PartialEq + Eq {}

impl Container<i64> {
    /// Find the maximum sub-array that crosses the mid point.
    pub fn find_max_crossing_subarray(
        &self,
        low: usize,
        mid: usize,
        high: usize,
    ) -> Option<MaxSubarray<i64>> {
        if self.is_empty() {
            return None;
        }

        let mut sum = 0;
        let mut left_sum = i64::MIN;
        let mut left_index = mid;

        for v in self.data[low..mid].iter().rev() {
            sum += v;
            if sum > left_sum {
                left_sum = sum;
                left_index -= 1;
            }
        }

        sum = 0;
        let mut right_sum = i64::MIN;
        let mut right_index = mid;
        for v in self.data[mid..high].iter() {
            sum += v;
            if sum > right_sum {
                right_sum = sum;
                right_index += 1;
            }
        }

        Some(MaxSubarray {
            left_index,
            right_index,
            sum: left_sum + right_sum,
        })
    }

    /// Returns maxmimum subarray.
    pub fn find_max_subarray(&self, low: usize, high: usize) -> Option<MaxSubarray<i64>> {
        if self.is_empty() {
            return None;
        } else if low == high - 1 {
            return Some(MaxSubarray {
                left_index: low,
                right_index: low,
                sum: self[low],
            });
        }

        let mid = (low + high) / 2;

        let left = self.find_max_subarray(low, mid);
        let right = self.find_max_subarray(mid, high);
        let cross = self.find_max_crossing_subarray(low, mid, high);

        if left > right && left > cross {
            left
        } else if right > left && right > cross {
            right
        } else {
            cross
        }
    }

    /// Returns maxmimum subarray.
    pub fn brute_force_max_subarray(&self, low: usize, high: usize) -> Option<MaxSubarray<i64>> {
        if self.is_empty() {
            return None;
        } else if low == high - 1 {
            return Some(MaxSubarray {
                left_index: low,
                right_index: low,
                sum: self[low],
            });
        }

        let mut max_sum = i64::MIN;
        let (mut left_index, mut right_index) = (0, 0);

        for i in low..high {
            let mut sum = self[i];
            for j in i + 1..high {
                sum += self[j];
                if sum > max_sum {
                    max_sum = sum;
                    (left_index, right_index) = (i, j);
                }
            }
        }

        Some(MaxSubarray {
            left_index,
            right_index,
            sum: max_sum,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_maximum_crossing_subarray() {
        let container = Container::new(vec![1, -2, 3, 1, -3, 7, 3]);
        let value = container.find_max_crossing_subarray(0, 3, 7);
        assert_eq!(
            Some(MaxSubarray {
                left_index: 2,
                right_index: 6,
                sum: 11
            }),
            value
        )
    }

    #[test]
    fn find_maximum_subarray() {
        let container = Container::new(vec![1, -2, 3, 1, -3, 7, 3]);
        let value = container.find_max_subarray(0, 7);
        assert_eq!(
            Some(MaxSubarray {
                left_index: 2,
                right_index: 6,
                sum: 11
            }),
            value
        )
    }

    #[test]
    fn brute_force_maximum_subarray() {
        let container = Container::new(vec![1, -2, 3, 1, -3, 7, 3]);
        let value = container.brute_force_max_subarray(0, 7);
        assert_eq!(
            Some(MaxSubarray {
                left_index: 2,
                right_index: 6,
                sum: 11
            }),
            value
        )
    }
}
