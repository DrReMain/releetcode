package n206

import (
	"reflect"
	"testing"

	"drremain.cn/playground-golang/DataStructure"
)

func TestReverseList1(t *testing.T) {
	input := DataStructure.NewListNodeFromSlice([]int{1, 2, 3, 4, 5})
	expected := DataStructure.NewListNodeFromSlice([]int{5, 4, 3, 2, 1})
	result := reverseList(input)
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %v, but got %v for input %v", expected, result, input)
	}
}

func TestReverseList2(t *testing.T) {
	input := DataStructure.NewListNodeFromSlice([]int{1, 2})
	expected := DataStructure.NewListNodeFromSlice([]int{2, 1})
	result := reverseList(input)
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %v, but got %v for input %v", expected, result, input)
	}
}

func BenchmarkReverseList(b *testing.B) {
	input := DataStructure.NewListNodeFromSlice([]int{1, 2, 3, 4, 5})
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		reverseList(input)
	}
}
