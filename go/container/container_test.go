package container

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestInsertionSort(t *testing.T) {
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
			tt.data.InsertionSort()
			assert.Equalf(t, tt.want, tt.data.Values(), "want %v got %v", tt.want, tt.data.Values())

		})
	}

}

func TestLinearSearch(t *testing.T) {
	tests := []struct {
		name     string
		haystack Container[int]
		needle   int
		want     int
	}{
		{"empty container", Container[int]{values: []int{}}, 2, -1},
		{"needle found", Container[int]{values: []int{1, 3, -1, 2}}, 3, 1},
		{"return smallest index", Container[int]{values: []int{1, 2, 1, 3}}, 1, 0},
		{"missing value", Container[int]{values: []int{1, 3, -1, 0}}, 7, -1},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			index := tt.haystack.LinearSearch(tt.needle)
			assert.Equal(t, index, tt.want)
		})
	}

}

func TestSelectionSort(t *testing.T) {
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
			tt.data.SelectionSort()
			assert.Equalf(t, tt.want, tt.data.Values(), "want %v got %v", tt.want, tt.data.Values())

		})
	}

}
