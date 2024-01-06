/*
 * @lc app=leetcode.cn id=156 lang=golang
 *
 * [156] 上下翻转二叉树
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func upsideDownBinaryTree(root *TreeNode) *TreeNode {
	if root == nil || root.Left == nil {
		return root
	}
	left, right := root.Left, root.Right
	ret := upsideDownBinaryTree(left)
	left.Left = right
	left.Right = root
	root.Left, root.Right = nil, nil
	return ret
}
// @lc code=end

