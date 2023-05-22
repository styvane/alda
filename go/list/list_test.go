package list

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestList(t *testing.T) {
	list := NewList[int]()
	list.Insert(1)
	node := list.head
	list.Insert(2)
	list.Insert(3)

	assert.Equal(t, list.Head().elem, 3)
	list.Delete(node)
	assert.Equal(t, list.Head().next.elem, 2)
	assert.Nil(t, list.Head().next.next)
	n := list.Search(3)
	assert.NotNil(t, n)
	assert.Equal(t, n.elem, 3)
}
