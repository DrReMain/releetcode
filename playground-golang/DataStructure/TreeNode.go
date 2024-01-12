package DataStructure

type treeNodeVal interface {
	int | float64 | string
}

type TreeNode[T treeNodeVal] struct {
	Val   T
	Left  *TreeNode[T]
	Right *TreeNode[T]
}
