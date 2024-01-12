/*
 * @lc app=leetcode.cn id=124 lang=golang
 *
 * [124] 二叉树中的最大路径和
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
func maxPathSum(root *TreeNode) int {
	maxSum := math.MinInt
	var findMaxPathSum func(*TreeNode) int
	findMaxPathSum = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		leftMax := max(findMaxPathSum(node.Left), 0)
		rightMax := max(findMaxPathSum(node.Right), 0)
		maxSum = max(maxSum, node.Val + leftMax + rightMax)
		return node.Val + max(leftMax, rightMax)
	}
	findMaxPathSum(root)
	return maxSum
}
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
// @lc code=end

