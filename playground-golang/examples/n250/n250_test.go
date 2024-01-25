package n538

import (
	"testing"

	"drremain.cn/playground-golang/DataStructure"
)

func TestCountUnivalSubtrees1(t *testing.T) {
	root := DataStructure.NewTreeNodeFromSlice([]interface{}{5, 1, 5, 5, 5, nil, 5})
	expected := 4
	if got := countUnivalSubtrees(root); got != expected {
		t.Errorf("Expected %v, but got %v for input %#v", expected, got, root)
	}
}

func TestCountUnivalSubtrees2(t *testing.T) {
	root := DataStructure.NewTreeNodeFromSlice([]interface{}{})
	expected := 0
	if got := countUnivalSubtrees(root); got != expected {
		t.Errorf("Expected %v, but got %v for input %#v", expected, got, root)
	}
}

func TestCountUnivalSubtrees3(t *testing.T) {
	root := DataStructure.NewTreeNodeFromSlice([]interface{}{5, 5, 5, 5, 5, nil, 5})
	expected := 6
	if got := countUnivalSubtrees(root); got != expected {
		t.Errorf("Expected %v, but got %v for input %#v", expected, got, root)
	}
}

func BenchmarkCountUnivalSubtrees(b *testing.B) {
	root := DataStructure.NewTreeNodeFromSlice([]interface{}{5, 1, 5, 5, 5, nil, 5})
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		countUnivalSubtrees(root)
	}
}
