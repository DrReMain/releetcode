package n3

import (
	"testing"
)

func TestLengthOfLongestSubstring1(t *testing.T) {
	input := "abcabcbb"
	expected := 3
	result := lengthOfLongestSubstring(input)
	if result != expected {
		t.Errorf("Expected %d, but got %d for input %s", expected, result, input)
	}
}

func TestLengthOfLongestSubstring2(t *testing.T) {
	input := "bbbbb"
	expected := 1
	result := lengthOfLongestSubstring(input)
	if result != expected {
		t.Errorf("Expected %d, but got %d for input %s", expected, result, input)
	}
}

func BenchmarkLengthOfLongestSubstring(b *testing.B) {
	input := "abcabcbb"
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		lengthOfLongestSubstring(input)
	}
}
