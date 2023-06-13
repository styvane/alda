// Package tree defines various tree data structures.
package tree

import (
	"golang.org/x/exp/constraints"
)

// Node is a node in a tree.
type Node[T constraints.Ordered] struct {
	Key                 T
	Parent, Left, Right *Node[T]
}

// BinaryTree represents a binary tree data structure.
type BinaryTree[T constraints.Ordered] struct {
	Root *Node[T]
}

// Insert new node with the given key into the a binary search tree.
func (b *BinaryTree[T]) Insert(key T) {
	var parent *Node[T]

	for c := b.Root; c != nil; {
		parent = c
		if c != nil && key < c.Key {
			c = c.Left
		} else {
			c = c.Right
		}
	}

	n := &Node[T]{Key: key}
	n.Parent = parent

	if parent == nil {
		b.Root = n
	}

	if parent != nil {
		if parent.Key > n.Key {
			parent.Left = n
		} else {
			parent.Right = n
		}
	}
}

func (n *Node[T]) inorder(keys []T) []T {
	if n != nil {
		keys = n.Left.inorder(keys)
		keys = append(keys, n.Key)
		keys = n.Right.inorder(keys)
	}

	return keys

}

// InOrderWalk returns all the keys in a binary search tree
// in sorted order.
func (b *BinaryTree[T]) InOrderWalk() []T {
	var keys []T
	return b.Root.inorder(keys)
}

func (n *Node[T]) postOrder(keys []T) []T {
	if n != nil {
		keys = n.Left.postOrder(keys)
		keys = n.Right.postOrder(keys)
		keys = append(keys, n.Key)

	}
	return keys

}

// PostOrderWalk returns the subtree left nodes keys followed by
// subtree right nodes keys, and the subtree root keys.
func (b *BinaryTree[T]) PostOrderWalk() []T {
	var keys []T
	return b.Root.postOrder(keys)
}

func (n *Node[T]) preOrder(keys []T) []T {
	if n != nil {
		keys = append(keys, n.Key)
		keys = n.Left.preOrder(keys)
		keys = n.Right.preOrder(keys)
	}
	return keys

}

// PreOrderWalk returns the subtree root nodes keys followed by
// subtree left nodes keys, and the subtree right keys.
func (b *BinaryTree[T]) PreOrderWalk() []T {
	var keys []T
	return b.Root.preOrder(keys)
}

// RecursiveSearch recursively find a node with a given key in the tree.
func (b *BinaryTree[T]) RecursiveSearch(key T) *Node[T] {
	var rec func(n *Node[T], k T) *Node[T]
	rec = func(n *Node[T], k T) *Node[T] {
		if n == nil || n.Key == key {
			return n
		} else if k < n.Key {
			return rec(n.Left, k)
		} else {
			return rec(n.Right, k)
		}
	}

	return rec(b.Root, key)
}

// Search find the node with a given key in the tree.
func (b *BinaryTree[T]) Search(key T) *Node[T] {
	n := b.Root
	for n != nil && n.Key != key {
		if key < n.Key {
			n = n.Left
		} else {
			n = n.Right
		}
	}
	return n
}

func (n *Node[T]) min() *Node[T] {
	for ; n != nil && n.Left != nil; n = n.Left {
	}

	return n
}

// Min find a node in a binary tree whose key is a minimum.
func (b *BinaryTree[T]) Min() *Node[T] {
	return b.Root.min()
}

func (n *Node[T]) max() *Node[T] {
	for ; n != nil && n.Right != nil; n = n.Right {
	}

	return n
}

// Max find a node in a binary tree whose key is a maximum.
func (b *BinaryTree[T]) Max() *Node[T] {
	return b.Root.max()
}

// Succ finds a node's successor in the tree.
func (b *BinaryTree[T]) Succ(k T) *Node[T] {
	n := b.Search(k)
	if n == nil {
		return n
	}

	if n.Right != nil {
		return n.min()
	}

	y := n.Parent
	for y != nil && n == y.Right {
		n = y
		y = y.Parent
	}

	return y
}

// Pred finds a node's predecessor in the tree/
func (b *BinaryTree[T]) Pred(k T) *Node[T] {
	n := b.Search(k)
	if n == nil {
		return n
	}

	if n.Left != nil {
		return n.max()
	}

	y := n.Parent
	for y != nil && n == y.Left {
		n = y
		y = y.Parent
	}

	return y
}

// transplant replaces one subtree as a child of its parent with
// another subtree.
func (b *BinaryTree[T]) transplant(u, v *Node[T]) {
	switch {
	case u.Parent == nil:
		b.Root = v
	case u == u.Parent.Left:
		u.Parent.Left = v
	default:
		u.Parent.Right = v
	}

	if v != nil {
		v.Parent = u.Parent
	}

}

// Delete a node with the given key from the tree
func (b *BinaryTree[T]) Delete(k T) {
	n := b.Search(k)
	switch {
	case n.Left == nil:
		b.transplant(n, n.Right)
	case n.Right == nil:
		b.transplant(n, n.Left)
	default:
		y := n.Right.min()
		if y.Parent != n {
			b.transplant(y, y.Right)
			y.Right = n.Right
			y.Right.Parent = y
		}

		b.transplant(n, y)
		y.Left = n.Left
		y.Left.Parent = y
	}
}
