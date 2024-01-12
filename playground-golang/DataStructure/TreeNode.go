package DataStructure

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func NewTreeNode(val int) *TreeNode {
	return &TreeNode{Val: val, Left: nil, Right: nil}
}

func NewTreeNodeWithNode(val int, left, right *TreeNode) *TreeNode {
	return &TreeNode{Val: val, Left: left, Right: right}
}

func NewTreeNodeFromSlice(values []interface{}) *TreeNode {
	return buildTree(values, 0)
}

func buildTree(values []interface{}, index int) *TreeNode {
	if index < len(values) {
		if val, ok := values[index].(int); ok {
			node := NewTreeNode(val)
			node.Left = buildTree(values, 2*index+1)
			node.Right = buildTree(values, 2*index+2)
			return node
		}
	}
	return nil
}
