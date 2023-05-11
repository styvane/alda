// Package queue implements queue data structure.
package queue

import "errors"

// BoundedQueue is a circular queue implemented using a slice as buffer.
type BoundedQueue[T any] struct {
	// The position of the element to dequeue.
	head int

	// The position of the next element to enqueue.
	tail int

	// The number of element in the queue.
	len int

	// The queue's buffer.
	buf []T

	// The maximal size of the queue.
	cap int
}

// NewBoundedQueue instantiate new bounded queue.
func NewBoundedQueue[T any](cap int) *BoundedQueue[T] {
	return &BoundedQueue[T]{head: 0, tail: 0, len: 0, buf: make([]T, 0, cap), cap: cap}
}

// IsEmpty returns true if the queue is empty, otherwise false.
func (b *BoundedQueue[T]) IsEmpty() bool {
	return len(b.buf) == 0
}

// Len returns the number of items in the queue.
func (b *BoundedQueue[T]) Len() int {
	return len(b.buf)
}

// IsFull returns true if the queue is full, otherwise false.
func (b *BoundedQueue[T]) IsFull() bool {
	return b.len == b.cap
}

// Enqueue insert new item to the queue
func (b *BoundedQueue[T]) Enqueue(elem T) error {
	if b.IsFull() {
		return errors.New("queue overflow")
	}

	if b.tail < b.len {
		b.buf[b.tail] = elem
	} else {
		b.buf = append(b.buf, elem)
	}
	b.tail = (b.tail + 1) % b.cap
	b.len += 1
	return nil
}

// Dequeue remove an item from the queue.
func (b *BoundedQueue[T]) Dequeue() (T, error) {
	var v T

	if b.IsEmpty() {
		return v, errors.New("underflow")
	}

	v = b.buf[b.head]
	b.len -= 1
	b.head = (b.head + 1) % b.cap
	return v, nil
}
