package n1

import (
	"reflect"
	"testing"
)

func TestTwoSum1(t *testing.T) {
	nums, target := []int{2, 7, 11, 15}, 9
	expected := []int{0, 1}
	result := twoSum(nums, target)
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %d, but got %d for input [%v, %d]", expected, result, nums, target)
	}
}

func TestTwoSum2(t *testing.T) {
	nums, target := []int{3, 2, 4}, 6
	expected := []int{1, 2}
	result := twoSum(nums, target)
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %d, but got %d for input [%v, %d]", expected, result, nums, target)
	}
}

func TestTwoSum3(t *testing.T) {
	nums, target := []int{3, 3}, 6
	expected := []int{0, 1}
	result := twoSum(nums, target)
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %d, but got %d for input [%v, %d]", expected, result, nums, target)
	}
}

func BenchmarkTwoSum(b *testing.B) {
	nums, target := []int{2, 7, 11, 15}, 9
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		twoSum(nums, target)
	}
}
