// Heap data structure

package heap

import (
	"golang.org/x/exp/constraints"
)

// The Heap type represents the heap data structure.
type Heap[T constraints.Integer] struct {
	size   int
	buffer []T
}

// Create new heap with the give buffer.
func With[T constraints.Integer](buffer []T) *Heap[T] {
	return &Heap[T]{buffer: buffer, size: len(buffer)}
}

// Returns the parent's index for the child at the specified index.
func (h *Heap[T]) Parent(index int) int {
	return index / 2
}

// Returns the left child index of the element rooted at a given index.
func (h *Heap[T]) LeftChild(index int) int {
	return index*2 + 1
}

// Returns the right child index of the element routed at a given index.
func (h *Heap[T]) RightChild(index int) int {
	return index*2 + 2
}

// Re-arrange the elements so that the subtree at the specified index
// so that the subtree rooted at the index satified the max heap property.
func (h *Heap[T]) MaxHeapify(index int) {
	left := h.LeftChild(index)
	right := h.RightChild(index)
	largest := index
	if left < h.size && h.buffer[left] > h.buffer[index] {
		largest = left
	}
	if right < h.size && h.buffer[right] > h.buffer[largest] {
		largest = right
	}
	if largest != index {
		h.buffer[index], h.buffer[largest] = h.buffer[largest], h.buffer[index]
		h.MaxHeapify(largest)
	}
}

// Build max heap.
func (h *Heap[T]) BuildMaxHeap() {
	for i := len(h.buffer) / 2; i > 0; i -= 1 {
		h.MaxHeapify(i)
	}
}

// Returns maximum value in the heap.
func (h *Heap[T]) Max() (bool, T) {
	if h.size == 0 {
		return false, 0
	}
	return true, h.buffer[0]
}

// Extracts the maximum value from the heap.
func (h *Heap[T]) ExtractMax() (bool, T) {
	if h.size == 0 {
		return false, 0
	}
	max := h.buffer[0]
	h.buffer[0], h.buffer[h.size-1] = h.buffer[h.size-1], h.buffer[0]
	h.buffer = h.buffer[:h.size-1]
	h.size -= 1
	h.MaxHeapify(0)
	return true, max
}

// Increases the value of a key in the heap.
func (h *Heap[T]) IncreaseKey(index int, key T) {
	if index >= h.size || index < 0 || h.buffer[index] <= key {
		return
	}
	h.buffer[index] = key
	for i := h.Parent(index); h.buffer[index] > h.buffer[i]; {
		h.buffer[i], h.buffer[index] = h.buffer[index], h.buffer[i]
		i = h.Parent(i)
	}
}

// Sorts the values in the heap in an increasing order.
func (h *Heap[T]) Sort() {
	h.BuildMaxHeap()
	for i := h.size - 1; i > 0; i -= 1 {
		h.buffer[0], h.buffer[i] = h.buffer[i], h.buffer[0]
		h.size -= 1
		h.MaxHeapify(0)
	}
}

// Re-arrange the elements in the heap so that the subtree rooted
// at the specified index statified the min heap property.
func (h *Heap[T]) MinHeapify(index int) {
	left := h.LeftChild(index)
	right := h.RightChild(index)
	smallest := index
	if left < h.size && h.buffer[left] < h.buffer[index] {
		smallest = left
	}
	if right < h.size && h.buffer[right] < h.buffer[smallest] {
		smallest = right
	}

	if smallest != index {
		h.buffer[index], h.buffer[smallest] = h.buffer[smallest], h.buffer[index]
		h.MinHeapify(smallest)
	}

}

// Build min heap
func (h *Heap[T]) BuildMinHeap() {
	for i := len(h.buffer) / 2; i > 0; i -= 1 {
		h.MinHeapify(i)
	}
}

// Insert new key into the heap.
func (h *Heap[T]) InsertKey(key T) {
	h.size += 1
	h.buffer[h.size] = key - 1
	h.IncreaseKey(h.size, key)
}
