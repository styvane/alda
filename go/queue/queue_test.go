package queue

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBoundedQueue(t *testing.T) {
	queue := NewBoundedQueue[int](5)
	assert.True(t, queue.IsEmpty())
	queue.Enqueue(1)
	queue.Enqueue(2)
	queue.Enqueue(3)
	queue.Enqueue(4)
	queue.Enqueue(5)
	assert.True(t, queue.IsFull())
	assert.Equal(t, queue.tail, 0)
	assert.NotNil(t, queue.Enqueue(6))
	val, _ := queue.Dequeue()
	assert.Equal(t, 1, val)
	assert.Equal(t, 1, queue.head)
	queue.Enqueue(6)
	assert.True(t, queue.IsFull())
	assert.Equal(t, 1, queue.tail)
	val, _ = queue.Dequeue()
	assert.Equal(t, 2, val)
	val, _ = queue.Dequeue()
	assert.Equal(t, 3, val)
	val, _ = queue.Dequeue()
	assert.Equal(t, 4, val)
	val, _ = queue.Dequeue()
	assert.Equal(t, 5, val)

}
