package n572

import (
	"testing"

	"drremain.cn/playground-golang/DataStructure"
)

func TestIsSubtree(t *testing.T) {
	root1 := DataStructure.NewTreeNodeFromSlice([]interface{}{3, 4, 5, 1, 2})
	subRoot1 := DataStructure.NewTreeNodeFromSlice([]interface{}{4, 1, 2})
	expected1, result1 := true, isSubtree(root1, subRoot1)
	if result1 != expected1 {
		t.Errorf("Expected %v, but got %v for input %#v %#v", expected1, result1, root1, subRoot1)
	}

	root2 := DataStructure.NewTreeNodeFromSlice([]interface{}{3, 4, 5, 1, 2, nil, nil, nil, nil, 0})
	subRoot2 := DataStructure.NewTreeNodeFromSlice([]interface{}{4, 1, 2})
	expected2, result2 := false, isSubtree(root2, subRoot2)
	if result2 != expected2 {
		t.Errorf("Expected %v, but got %v for input %#v %#v", expected2, result2, root2, subRoot2)
	}
}

func BenchmarkIsSubtree(b *testing.B) {
	root := DataStructure.NewTreeNodeFromSlice([]interface{}{3, 4, 5, 1, 2})
	subRoot := DataStructure.NewTreeNodeFromSlice([]interface{}{4, 1, 2})
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		isSubtree(root, subRoot)
	}
}
