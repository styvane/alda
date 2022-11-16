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
