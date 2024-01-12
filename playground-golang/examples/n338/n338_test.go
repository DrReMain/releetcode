package n338

import (
	"reflect"
	"testing"
)

func TestCountBits(t *testing.T) {
	tests := []struct {
		name string
		n    int
		want []int
	}{
		{
			name: "Test 1",
			n:    2,
			want: []int{0, 1, 1},
		},
		{
			name: "Test 2",
			n:    5,
			want: []int{0, 1, 1, 2, 1, 2},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := countBits(tt.n); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("countBits() = %v, want %v", got, tt.want)
			}
		})
	}
}

func BenchmarkCountBits(b *testing.B) {
	for i := 0; i < b.N; i++ {
		countBits(1000)
	}
}
