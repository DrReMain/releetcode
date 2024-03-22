package n25

import (
	"reflect"
	"testing"

	"drremain.cn/playground-golang/DataStructure"
)

func TestReverseKGroup1(t *testing.T) {
	head := DataStructure.NewListNodeFromSlice([]int{1, 2, 3, 4, 5})
	k := 2
	expected := DataStructure.NewListNodeFromSlice([]int{2, 1, 4, 3, 5})
	result := reverseKGroup(head, k)
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %v, but got %v for input [%v %v]", expected, result, head, k)
	}
}

func TestReverseKGroup2(t *testing.T) {
	head := DataStructure.NewListNodeFromSlice([]int{1, 2, 3, 4, 5})
	k := 3
	expected := DataStructure.NewListNodeFromSlice([]int{3, 2, 1, 4, 5})
	result := reverseKGroup(head, k)
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %v, but got %v for input [%v %v]", expected, result, head, k)
	}
}

func BenchmarkReverseKGroup(b *testing.B) {
	head := DataStructure.NewListNodeFromSlice([]int{1, 2, 3, 4, 5})
	k := 2
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		reverseKGroup(head, k)
	}
}
