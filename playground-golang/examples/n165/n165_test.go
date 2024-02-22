package n165

import (
	"testing"
)

func TestCompareVersion1(t *testing.T) {
	version1, version2 := "1.01", "1.001"
	result := compareVersion(version1, version2)
	expected := 0
	if result != expected {
		t.Errorf("Expected %d, but got %d for input [%s, %s]", expected, result, version1, version2)
	}
}

func TestCompareVersion2(t *testing.T) {
	version1, version2 := "1.0", "1.0.0"
	result := compareVersion(version1, version2)
	expected := 0
	if result != expected {
		t.Errorf("Expected %d, but got %d for input [%s, %s]", expected, result, version1, version2)
	}
}

func TestCompareVersion3(t *testing.T) {
	version1, version2 := "0.1", "1.1"
	result := compareVersion(version1, version2)
	expected := -1
	if result != expected {
		t.Errorf("Expected %d, but got %d for input [%s, %s]", expected, result, version1, version2)
	}
}

func BenchmarkCompareVersion(b *testing.B) {
	version1, version2 := "1.01", "1.001"
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		compareVersion(version1, version2)
	}
}
