package n28

import (
	"testing"
)

func TestStrStr(t *testing.T) {
	haystack1, needle1, expected1 := "sadbutsad", "sad", 0
	result1 := strStr(haystack1, needle1)
	if result1 != expected1 {
		t.Errorf("Expected %d, but got %d for input [%s %s]", expected1, result1, haystack1, needle1)
	}

	haystack2, needle2, expected2 := "leetcode", "leeto", -1
	result2 := strStr(haystack2, needle2)
	if result2 != expected2 {
		t.Errorf("Expected %d, but got %d for input [%s %s]", expected2, result2, haystack2, needle2)
	}
}

func BenchmarkStrStr(b *testing.B) {
	haystack, needle := "sadbutsad", "sad"
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		strStr(haystack, needle)
	}
}
