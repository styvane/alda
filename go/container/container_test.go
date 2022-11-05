package container

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestClrsInsertionSort(t *testing.T) {
	tests := []struct {
		name string
		data Container[int]
		want []int
	}{
		{name: "empty container", data: Container[int]{values: []int{}}, want: []int{}},
		{name: "many values", data: Container[int]{values: []int{1, 2, -9, 0}}, want: []int{-9, 0, 1, 2}},
		{name: "one element", data: Container[int]{values: []int{2}}, want: []int{2}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			tt.data.Clrs_insertion()
			assert.Equalf(t, tt.want, tt.data.Values(), "want %v got %v", tt.want, tt.data.Values())

		})
	}

}
