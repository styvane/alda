// Package stack define various stack data structures.
package stack

import "errors"

// The SimpleStack type is a wraps a slice of T.
type SimpleStack[T any] struct {
	buf []T
	top int
}

// NewSimpleStack instantiates new simple stack
func NewSimpleStack[T any]() *SimpleStack[T] {
	return &SimpleStack[T]{buf: make([]T, 0), top: 0}
}

// Checks if the stack is empty.
func (s *SimpleStack[T]) IsEmpty() bool {
	return s.top == 0
}

// Push pushes the elem T onto the stack.
func (s *SimpleStack[T]) Push(elem T) {
	s.top += 1
	s.buf = append(s.buf, elem)
}

// Pop pops an element from the stack.
func (s *SimpleStack[T]) Pop() (*T, error) {
	if s.IsEmpty() {
		return nil, errors.New("underflow")
	}
	s.top -= 1
	val := s.buf[s.top]
	s.buf = s.buf[:s.top]
	return &val, nil
}
