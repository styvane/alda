package stack

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSimpleStack(t *testing.T) {
	s := NewSimpleStack[int]()
	assert.Equal(t, s.top, 0)
	assert.True(t, s.IsEmpty())
	s.Push(1)
	s.Push(2)
	s.Push(3)
	v, _ := s.Pop()
	assert.Equal(t, *v, 3)
	s.Pop()
	s.Pop()
	_, err := s.Pop()
	assert.NotNil(t, err)

}
