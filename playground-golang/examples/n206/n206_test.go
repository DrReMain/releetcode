package n206

import (
	"reflect"
	"testing"
)

func TestReverseList(t *testing.T) {
	input1 := &ListNode{1, &ListNode{2, &ListNode{3, &ListNode{4, &ListNode{5, nil}}}}}
	expected1 := &ListNode{5, &ListNode{4, &ListNode{3, &ListNode{2, &ListNode{1, nil}}}}}
	result1 := reverseList(input1)
	if !reflect.DeepEqual(result1, expected1) {
		t.Errorf("Expected %v, but got %v for input %v", expected1, result1, input1)
	}

	input2 := &ListNode{1, &ListNode{2, nil}}
	expected2 := &ListNode{2, &ListNode{1, nil}}
	result2 := reverseList(input2)
	if !reflect.DeepEqual(result2, expected2) {
		t.Errorf("Expected %v, but got %v for input %v", expected2, result2, input2)
	}
}

func BenchmarkReverseList(b *testing.B) {
	input := &ListNode{1, &ListNode{2, &ListNode{3, &ListNode{4, &ListNode{5, nil}}}}}
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		reverseList(input)
	}
}
