// Package data defines a container data structure.
package container

import (
	"golang.org/x/exp/constraints"
)

// Container is a wrapper around around a generic slice over some type T.
type Container[T constraints.Ordered] struct {
	values []T
}

// Returns the underline slice of elements in the container.
func (c *Container[T]) Values() []T {
	return c.values
}

// InsertionSort implements Cormen, Leiserson, Rivest, Stein
// insertion sort algorithm.
func (c *Container[T]) InsertionSort() {
	if len(c.values) <= 1 {
		return
	}

	for j, key := range c.values[1:] {
		j += 1
		for i := j - 1; i >= 0 && c.values[i] > key; i -= 1 {
			c.values[i+1], c.values[i] = c.values[i], c.values[i+1]
		}
	}
}

// Linearly search a value in a container.
//
// It returns the smallest index of the value if it is present,
// otherwise, it returns -1
func (c *Container[T]) LinearSearch(needle T) int {
	for i, v := range c.values {
		if v == needle {
			return i
		}
	}
	return -1
}

// SelectionSort implements the selection sort algorithm.
func (c *Container[T]) SelectionSort() {
	if len(c.values) <= 1 {
		return
	}

	for i := range c.values[:len(c.values)-1] {
		min := i
		for j := i + 1; j < len(c.values); j += 1 {
			if c.values[j] < c.values[min] {
				min = j
			}
		}
		c.values[min], c.values[i] = c.values[i], c.values[min]
	}
}

// MergeSort implements the merge sort algorithm.
func (c *Container[T]) MergeSort(start, end int) {
	if start < end-1 {
		middle := (start + end) / 2
		c.MergeSort(start, middle)
		c.MergeSort(middle, end)
		c.merge(start, middle, end)
	}
}

// Merge two sorted container values.
func (c *Container[T]) merge(start, middle, end int) {
	left := make([]T, middle-start)
	copy(left, c.values[start:middle])
	right := make([]T, end-middle)
	copy(right, c.values[middle:end])

	for i, j, k := 0, 0, start; k < end; k += 1 {
		if i < len(left) && j < len(right) && left[i] <= right[j] || j == len(right) {
			c.values[k] = left[i]
			i += 1
		} else {
			c.values[k] = right[j]
			j += 1
		}
	}

}

// RecursiveInsort sorts a container elements using insertion sort.
// It recursively sorts the first N-1 elements in the container and
// insert the N-th element into its correct position.
func (c *Container[T]) RecursiveInsort() {
	var rec_sort func(values []T)
	rec_sort = func(values []T) {
		if len(values) <= 1 {
			return
		}

		n := len(values) - 1
		key := values[n]

		rec_sort(values[:n])
		for n := n - 1; n >= 0; n -= 1 {
			if key < values[n] {
				values[n+1], values[n] = values[n], values[n+1]
			}
		}

	}

	rec_sort(c.values)

}

// BinSearch searches a value in a sorted container.
// It returns the index of the value if the value is present.
// If the value is not present, it returns -1
func (c *Container[T]) BinSearch(needle T) int {
	if len(c.values) == 0 {
		return -1
	}

	for low, high := 0, len(c.values)-1; low <= high; {
		mid := (low + high) / 2
		if c.values[mid] == needle {
			return mid
		} else if c.values[mid] > needle {
			high = mid - 1
		} else {
			low = mid + 1
		}
	}
	return -1
}

// Recursively binary search a value in a sorted container.
func (c *Container[T]) RecBinSearch(needle T) int {
	var rec func(values []T, needle T) int

	rec = func(values []T, needle T) int {
		if len(values) == 0 {
			return -1
		}
		mid := (len(values) - 1) / 2
		if needle == values[mid] {
			return mid
		} else if needle > values[mid] {
			return rec(values[mid+1:], needle)
		} else {
			return rec(values[:mid], needle)
		}

	}
	return rec(c.values, needle)
}

type Number[T constraints.Signed] struct {
	values []T
}

// Find the maximum subarray of a list of values.
func (n *Number[T]) FindMaximumSubArray(low, high int) (left, right int, sum T) {
	if len(n.values) == 0 || low == high-1 {
		return
	}

	mid := (low + high) / 2

	left_min, left_max, left_sum := n.FindMaximumSubArray(low, mid)
	right_min, right_max, right_sum := n.FindMaximumSubArray(mid, high)
	cross_min, cross_max, cross_sum := n.findMaximumXSubArray(low, mid, high)
	if left_sum > right_sum && left_sum > cross_sum {
		return left_min, left_max, left_sum
	} else if right_sum > left_sum && right_sum > cross_sum {
		return right_min, right_max, right_sum
	} else {
		return cross_min, cross_max, cross_sum
	}
}

// Find the maximum  crossing subarray.
func (n *Number[T]) findMaximumXSubArray(low, mid, high int) (int, int, T) {
	var sum, left_sum T
	left_ix := 0

	for i := mid - 1; i >= low; i -= 1 {
		sum += n.values[i]
		if sum > left_sum {
			left_sum = sum
			left_ix = i
		}
	}

	sum = 0
	var right_sum T
	right_ix := 0

	for i := mid; i < high; i += 1 {
		sum += n.values[i]
		if sum > right_sum {
			right_sum = sum
			right_ix = i
		}
	}

	return left_ix, right_ix, left_sum + right_sum
}

// Find the maximum crossing subarray using brute force.
func (n *Number[T]) BruteForceMaximumSubArray(low, high int) (left_ix, right_ix int, max_sum T) {
	for i := low; i < high; i += 1 {
		sum := n.values[i]
		for j := i + 1; j < high; j += 1 {
			sum += n.values[j]
			if sum > max_sum {
				max_sum = sum
				left_ix = i
				right_ix = j

			}
		}
	}
	return
}
