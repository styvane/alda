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
			assert.Equal(t, tt.want, index)
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

func TestMergeSort(t *testing.T) {
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
			tt.data.MergeSort(0, len(tt.data.values))
			assert.Equalf(t, tt.want, tt.data.Values(), "want %v got %v", tt.want, tt.data.Values())

		})
	}

}

func TestRecursiveInsertionSort(t *testing.T) {
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
			tt.data.RecursiveInsort()
			assert.Equalf(t, tt.want, tt.data.Values(), "want %v got %v", tt.want, tt.data.Values())

		})
	}

}

func TestBinSearch(t *testing.T) {
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
			index := tt.haystack.BinSearch(tt.needle)
			assert.Equal(t, tt.want, index)
		})
	}

}

func TestRecBinSearch(t *testing.T) {
	tests := []struct {
		name     string
		haystack Container[int]
		needle   int
		want     int
	}{
		{"empty container", Container[int]{values: []int{}}, 2, -1},
		{"needle found", Container[int]{values: []int{1, 3, -1, 2}}, 3, 1},
		{"return smallest index", Container[int]{values: []int{1, 4, 2, 2, 1, 3}}, 2, 2},
		{"missing value", Container[int]{values: []int{1, 3, -1, 0}}, 7, -1},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			index := tt.haystack.RecBinSearch(tt.needle)
			assert.Equal(t, tt.want, index)
		})
	}

}

func TestMaximumCrossing(t *testing.T) {
	number := Number[int]{values: []int{1, -2, 3, 1, -3, 7, 3}}
	left, right, sum := number.findMaximumXSubArray(0, 3, 7)
	assert.Equal(t, left, 2)
	assert.Equal(t, right, 6)
	assert.Equal(t, sum, 11)
}

func TestMaximumSubArray(t *testing.T) {
	number := Number[int]{values: []int{1, -2, 3, 1, -3, 7, 3}}
	left, right, sum := number.FindMaximumSubArray(0, 6)
	assert.Equal(t, left, 2)
	assert.Equal(t, right, 5)
	assert.Equal(t, sum, 8)
}

func TestBruteForceMaximumSubArray(t *testing.T) {
	number := Number[int]{values: []int{1, -2, 3, 1, -3, 7, 3}}
	left, right, sum := number.BruteForceMaximumSubArray(0, 6)
	assert.Equal(t, left, 2)
	assert.Equal(t, right, 5)
	assert.Equal(t, sum, 8)
}
