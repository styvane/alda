package heap

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMaxHeapify(t *testing.T) {
	heap := With([]int{16, 4, 10, 14, 7, 9, 3, 2, 8, 1})
	heap.size = len(heap.buffer)
	heap.MaxHeapify(1)
	assert.Equal(t, heap.buffer, []int{16, 14, 10, 8, 7, 9, 3, 2, 4, 1})
}

func TestBuildMaxHeap(t *testing.T) {
	heap := With([]int{16, 4, 10, 14, 7, 9, 3, 2, 8, 1})
	heap.size = len(heap.buffer)
	heap.BuildMaxHeap()
	assert.Equal(t, heap.buffer, []int{16, 14, 10, 8, 7, 9, 3, 2, 4, 1})
}

func TestExtractMaxHeap(t *testing.T) {
	heap := With([]int{16, 4, 10, 14, 7, 9, 3, 2, 8, 1})
	heap.BuildMaxHeap()
	ok, v := heap.ExtractMax()
	assert.True(t, ok)
	assert.Equal(t, 16, v)
	assert.Equal(t, heap.buffer, []int{14, 8, 10, 4, 7, 9, 3, 2, 1})
}

func TestSort(t *testing.T) {
	heap := With([]int{16, 4, 10, 14, 7, 9, 3, 2, 8, 1})
	heap.Sort()
	assert.Equal(t, []int{1, 2, 3, 4, 7, 8, 9, 10, 14, 16}, heap.buffer)
}
