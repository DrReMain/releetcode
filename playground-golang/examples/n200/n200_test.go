package n200

import (
	"testing"
)

func TestNumIslands1(t *testing.T) {
	grid := [][]byte{
		{'1', '1', '1', '1', '0'},
		{'1', '1', '0', '1', '0'},
		{'1', '1', '0', '0', '0'},
		{'0', '0', '0', '0', '0'},
	}
	expected := 1
	result := numIslands(grid)
	if result != expected {
		t.Errorf("Expected %d, but got %d for input %#v", expected, result, grid)
	}
}

func TestNumIslands2(t *testing.T) {
	grid := [][]byte{
		{'1', '1', '0', '0', '0'},
		{'1', '1', '0', '0', '0'},
		{'0', '0', '1', '0', '0'},
		{'0', '0', '0', '1', '1'},
	}
	expected := 3
	result := numIslands(grid)
	if result != expected {
		t.Errorf("Expected %d, but got %d for input %#v", expected, result, grid)
	}
}

func BenchmarkNumIslands1(b *testing.B) {
	grid := [][]byte{
		{'1', '1', '1', '1', '0'},
		{'1', '1', '0', '1', '0'},
		{'1', '1', '0', '0', '0'},
		{'0', '0', '0', '0', '0'},
	}
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		numIslands(grid)
	}
}
