//! Bits manipulation algorithms

/// Integer bits addition trait.
trait Add<const N: usize, const M: usize, Rhs = Self> {
    type Output;
    /// Add two N-bits binary integer.
    fn add(&self, rhs: Rhs) -> Self::Output;
}

#[derive(Debug, Eq, PartialEq)]
struct BitArray<const N: usize>([usize; N]);

impl<const N: usize, const M: usize> Add<N, M> for BitArray<N> {
    type Output = BitArray<M>;
    fn add(&self, rhs: Self) -> Self::Output {
        let mut result = BitArray([0; M]);
        let mut carry = 0;
        for index in (0..N).rev() {
            let sum = self.0[index] + rhs.0[index] + carry;
            result.0[index] = sum % 2;
            carry = sum.div_euclid(2);
        }

        if carry != 0 {
            result.0.as_mut_slice().rotate_right(1);
            result.0[0] = carry;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_bit_array_with_result_of_same_length() {
        let arr1 = BitArray([0, 1, 0, 1, 1]);
        let arr2 = BitArray([0, 1, 0, 1, 1]);
        let res = arr1.add(arr2);
        assert_eq!(res, BitArray([1, 0, 1, 1, 0]))
    }

    #[test]
    fn add_two_bit_array_with_result_of_larger_length() {
        let arr1 = BitArray([1, 1, 1, 1, 1]);
        let arr2 = BitArray([0, 1, 0, 1, 1]);
        let res = arr1.add(arr2);
        assert_eq!(res, BitArray([1, 0, 1, 0, 1, 0]))
    }
}
