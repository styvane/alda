// Package list defines linked-list data structure.
package list

// List represents a single linked list data structure.
type List[T comparable] struct {
	head *Node[T]
}

// Node is a node in the linked list.
type Node[T comparable] struct {
	elem T
	next *Node[T]
}

// Creates new node.
func NewNode[T comparable](elem T) *Node[T] {
	node := new(Node[T])
	node.elem = elem
	return node
}

// Create new list.
func NewList[T comparable]() *List[T] {
	return &List[T]{}
}

// Insert new node to the list.
//
// It inserts the new element by splicing the list on the head.
func (l *List[T]) Insert(elem T) {
	node := Node[T]{elem, nil}
	node.next = l.head
	l.head = &node
}

// Head returns the value of the head node.
func (l *List[T]) Head() *Node[T] {
	return l.head
}

// Deletes a node from the list.
func (l *List[T]) Delete(node *Node[T]) {
	prev := l.head

	for c := prev; c != nil && c != node; c = c.next {
		prev = c
	}

	prev.next = node.next
}

// Search find a node with the given element in the list.
func (l *List[T]) Search(elem T) *Node[T] {
	var c *Node[T]

	for c = l.head; c != nil && c.elem != elem; c = c.next {
		c = c.next
	}
	return c
}
