package n3

import (
	"testing"
)

func TestLengthOfLongestSubstring(t *testing.T) {
	input1 := "abcabcbb"
	expected1 := 3
	result1 := lengthOfLongestSubstring(input1)
	if result1 != expected1 {
		t.Errorf("Expected %d, but got %d for input %s", expected1, result1, input1)
	}

	input2 := "bbbbb"
	expected2 := 1
	result2 := lengthOfLongestSubstring(input2)
	if result2 != expected2 {
		t.Errorf("Expected %d, but got %d for input %s", expected2, result2, input2)
	}
}

func BenchmarkLengthOfLongestSubstring(b *testing.B) {
	input := "abcabcbb"
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		lengthOfLongestSubstring(input)
	}
}
