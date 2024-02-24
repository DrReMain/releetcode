package n239

import (
	"reflect"
	"testing"
)

func TestMaxSlidingWindow1(t *testing.T) {
	nums, k := []int{1, 3, -1, -3, 5, 3, 6, 7}, 3
	result := maxSlidingWindow(nums, k)
	expected := []int{3, 3, 5, 5, 6, 7}
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %d, but got %d for input [%v, %d]", expected, result, nums, k)
	}
}

func TestMaxSlidingWindow2(t *testing.T) {
	nums, k := []int{1}, 1
	result := maxSlidingWindow(nums, k)
	expected := []int{1}
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %d, but got %d for input [%v, %d]", expected, result, nums, k)
	}
}

func BenchmarkMaxSlidingWindow(b *testing.B) {
	nums, k := []int{1, 3, -1, -3, 5, 3, 6, 7}, 3
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		maxSlidingWindow(nums, k)
	}
}
