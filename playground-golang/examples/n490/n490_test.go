package n490

import (
	"reflect"
	"testing"
)

func TestHasPath1(t *testing.T) {
	maze, start, destination := [][]int{
		{0, 0, 1, 0, 0},
		{0, 0, 0, 0, 0},
		{0, 0, 0, 1, 0},
		{1, 1, 0, 1, 1},
		{0, 0, 0, 0, 0},
	}, []int{0, 4}, []int{4, 4}

	expected := true
	result := hasPath(maze, start, destination)
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %v, but got %v for input [%v, %v, %v]", expected, result, maze, start, destination)
	}
}

func TestHasPath2(t *testing.T) {
	maze, start, destination := [][]int{
		{0, 0, 1, 0, 0},
		{0, 0, 0, 0, 0},
		{0, 0, 0, 1, 0},
		{1, 1, 0, 1, 1},
		{0, 0, 0, 0, 0},
	}, []int{0, 4}, []int{3, 2}

	expected := false
	result := hasPath(maze, start, destination)
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %v, but got %v for input [%v, %v, %v]", expected, result, maze, start, destination)
	}
}

func TestHasPath3(t *testing.T) {
	maze, start, destination := [][]int{
		{0, 0, 0, 0, 0},
		{1, 1, 0, 0, 1},
		{0, 0, 0, 0, 0},
		{0, 1, 0, 0, 1},
		{0, 1, 0, 0, 0},
	}, []int{4, 3}, []int{0, 1}

	expected := false
	result := hasPath(maze, start, destination)
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %v, but got %v for input [%v, %v, %v]", expected, result, maze, start, destination)
	}
}

func BenchmarkHasPath(b *testing.B) {
	maze, start, destination := [][]int{
		{0, 0, 1, 0, 0},
		{0, 0, 0, 0, 0},
		{0, 0, 0, 1, 0},
		{1, 1, 0, 1, 1},
		{0, 0, 0, 0, 0},
	}, []int{0, 4}, []int{4, 4}
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		hasPath(maze, start, destination)
	}
}
