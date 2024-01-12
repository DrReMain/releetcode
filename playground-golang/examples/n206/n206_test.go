package n206

import (
	"reflect"
	"testing"

	"drremain.cn/playground-golang/DataStructure"
)

func TestReverseList(t *testing.T) {
	input1 := DataStructure.NewListNodeFromSlice([]int{1, 2, 3, 4, 5})
	expected1 := DataStructure.NewListNodeFromSlice([]int{5, 4, 3, 2, 1})
	result1 := reverseList(input1)
	if !reflect.DeepEqual(result1, expected1) {
		t.Errorf("Expected %v, but got %v for input %v", expected1, result1, input1)
	}

	input2 := DataStructure.NewListNodeFromSlice([]int{1, 2})
	expected2 := DataStructure.NewListNodeFromSlice([]int{2, 1})
	result2 := reverseList(input2)
	if !reflect.DeepEqual(result2, expected2) {
		t.Errorf("Expected %v, but got %v for input %v", expected2, result2, input2)
	}
}

func BenchmarkReverseList(b *testing.B) {
	input := DataStructure.NewListNodeFromSlice([]int{1, 2, 3, 4, 5})
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		reverseList(input)
	}
}
