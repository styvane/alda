// Package data defines a container data structure.
package container

import "golang.org/x/exp/constraints"

// Container is a wrapper around around a generic slice over some type T.
type Container[T constraints.Ordered] struct {
	values []T
}

// Returns the underline slice of elements in the container.
func (c *Container[T]) Values() []T {
	return c.values
}

// Clrs_insertion implement Cormen, Leiserson, Rivest, Stein
// insertion sort algorithm.
func (c *Container[T]) Clrs_insertion() {
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