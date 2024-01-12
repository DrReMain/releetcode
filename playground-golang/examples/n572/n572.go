package n572

import "drremain.cn/playground-golang/DataStructure"

type TreeNode = DataStructure.TreeNode

func isSubtree(root *TreeNode, subRoot *TreeNode) bool {
	if root == nil {
		return false
	}
	return check(root, subRoot) || isSubtree(root.Left, subRoot) || isSubtree(root.Right, subRoot)
}

func check(t1 *TreeNode, t2 *TreeNode) bool {
	if t1 == nil && t2 == nil {
		return true
	}
	if t1 == nil || t2 == nil {
		return false
	}
	if t1.Val == t2.Val {
		return check(t1.Left, t2.Left) && check(t1.Right, t2.Right)
	}
	return false
}
