package n62

import (
	"testing"
)

func TestUniquePaths1(t *testing.T) {
	m, n := 3, 7
	expected := 28
	result := uniquePaths(m, n)
	if result != expected {
		t.Errorf("Expected %d, but got %d for input [%d, %d]", expected, result, m, n)
	}
}

func TestUniquePaths2(t *testing.T) {
	m, n := 3, 2
	expected := 3
	result := uniquePaths(m, n)
	if result != expected {
		t.Errorf("Expected %d, but got %d for input [%d, %d]", expected, result, m, n)
	}
}

func TestUniquePaths3(t *testing.T) {
	m, n := 7, 3
	expected := 28
	result := uniquePaths(m, n)
	if result != expected {
		t.Errorf("Expected %d, but got %d for input [%d, %d]", expected, result, m, n)
	}
}

func TestUniquePaths4(t *testing.T) {
	m, n := 3, 3
	expected := 6
	result := uniquePaths(m, n)
	if result != expected {
		t.Errorf("Expected %d, but got %d for input [%d, %d]", expected, result, m, n)
	}
}

func BenchmarkUniquePaths(b *testing.B) {
	m, n := 3, 7
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		uniquePaths(m, n)
	}
}
