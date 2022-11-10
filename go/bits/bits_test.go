package bits

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBits(t *testing.T) {
	tests := []struct {
		first  BitArray
		second BitArray
		want   BitArray
		name   string
	}{

		{
			first:  BitArray{0, 1, 0, 1, 1},
			second: BitArray{0, 1, 0, 1, 1},
			want:   BitArray{1, 0, 1, 1, 0},
			name:   "same_length",
		},

		{
			first:  BitArray{1, 1, 1, 1, 1},
			second: BitArray{0, 1, 0, 1, 1},
			want:   BitArray{1, 0, 1, 0, 1, 0},
			name:   "result_has_bigger_size",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			res := tt.first.Add(tt.second)
			assert.Equal(t, res, tt.want)
		})
	}

}
