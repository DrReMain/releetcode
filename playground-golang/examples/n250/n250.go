package n538

import "drremain.cn/playground-golang/DataStructure"

type TreeNode = DataStructure.TreeNode

func countUnivalSubtrees(root *TreeNode) (ret int) {
	if root == nil {
		return
	}
	var isUnivalSubtrees func(*TreeNode, int) bool
	isUnivalSubtrees = func(node *TreeNode, value int) bool {
		if node == nil {
			return true
		}
		left := isUnivalSubtrees(node.Left, node.Val)
		right := isUnivalSubtrees(node.Right, node.Val)

		if left && right {
			ret++
			return node.Val == value
		} else {
			return false
		}
	}
	isUnivalSubtrees(root, root.Val)
	return
}
