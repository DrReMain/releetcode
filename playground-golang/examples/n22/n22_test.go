package n22

import (
	"reflect"
	"testing"
)

func TestGenerateParenthesis1(t *testing.T) {
	n := 3
	expected := []string{"((()))", "(()())", "(())()", "()(())", "()()()"}
	result := generateParenthesis(n)
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %v, but got %v for input %v", expected, result, n)
	}
}

func TestGenerateParenthesis2(t *testing.T) {
	n := 1
	expected := []string{"()"}
	result := generateParenthesis(n)
	if !reflect.DeepEqual(result, expected) {
		t.Errorf("Expected %v, but got %v for input %v", expected, result, n)
	}
}

func BenchmarkGenerateParenthesis(b *testing.B) {
	n := 3
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		generateParenthesis(n)
	}
}
