package tree

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBinaryTree(t *testing.T) {
	tree := BinaryTree[int]{}

	values := []int{6, 4, 7, 2, 5, 8}

	for _, k := range values {
		tree.Insert(k)
	}

	assert.Equal(t, tree.Root.Key, 6)
	assert.Equal(t, tree.Root.Left.Key, 4)
	assert.Equal(t, tree.Root.Right.Key, 7)
	seven := tree.Root.Right
	assert.Nil(t, seven.Left)

	keys := tree.InOrderWalk()
	assert.Equal(t, []int{2, 4, 5, 6, 7, 8}, keys)

	assert.Equal(t, []int{2, 4, 5, 6, 7, 8}, keys)

	keys = tree.PostOrderWalk()
	assert.Equal(t, []int{2, 5, 4, 8, 7, 6}, keys)

	keys = tree.PreOrderWalk()
	assert.Equal(t, []int{6, 4, 2, 5, 7, 8}, keys)

	n := tree.RecursiveSearch(7)
	assert.NotNil(t, n)
	assert.Equal(t, n.Right.Key, 8)

	n = tree.Search(7)
	assert.NotNil(t, n)
	assert.Equal(t, 8, n.Right.Key)

	n = tree.Min()
	assert.Equal(t, 2, n.Key)

	n = tree.Max()
	assert.Equal(t, 8, n.Key)

	n = tree.Succ(5)
	assert.Equal(t, 6, n.Key)
	assert.Nil(t, tree.Succ(8))

	assert.Nil(t, tree.Pred(2))
	assert.Equal(t, 6, tree.Pred(7).Key)

	tree.Delete(4)
	assert.Equal(t, 5, tree.Root.Left.Key)
	tree.Delete(6)
	assert.Equal(t, 7, tree.Root.Key)

}
