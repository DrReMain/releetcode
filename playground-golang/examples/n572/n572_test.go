package n572

import "testing"

func TestIsSubtree(t *testing.T) {
	root1 := &TreeNode{
		3,
		&TreeNode{
			4,
			&TreeNode{1, nil, nil},
			&TreeNode{2, nil, nil},
		},
		&TreeNode{5, nil, nil},
	}
	subRoot1 := &TreeNode{
		4,
		&TreeNode{1, nil, nil},
		&TreeNode{2, nil, nil},
	}
	expected1, result1 := true, isSubtree(root1, subRoot1)
	if result1 != expected1 {
		t.Errorf("Expected %v, but got %v for input %#v %#v", expected1, result1, root1, subRoot1)
	}

	root2 := &TreeNode{
		3,
		&TreeNode{
			4,
			&TreeNode{1, nil, nil},
			&TreeNode{2, &TreeNode{0, nil, nil}, nil},
		},
		&TreeNode{5, nil, nil},
	}
	subRoot2 := &TreeNode{
		4,
		&TreeNode{1, nil, nil},
		&TreeNode{2, nil, nil},
	}
	expected2, result2 := false, isSubtree(root2, subRoot2)
	if result2 != expected2 {
		t.Errorf("Expected %v, but got %v for input %#v %#v", expected2, result2, root2, subRoot2)
	}
}

func BenchmarkIsSubtree(b *testing.B) {
	root := &TreeNode{
		3,
		&TreeNode{
			4,
			&TreeNode{1, nil, nil},
			&TreeNode{2, nil, nil},
		},
		&TreeNode{5, nil, nil},
	}
	subRoot := &TreeNode{
		4,
		&TreeNode{1, nil, nil},
		&TreeNode{2, nil, nil},
	}
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		isSubtree(root, subRoot)
	}
}
