/*
 * @lc app=leetcode.cn id=42 lang=golang
 *
 * [42] 接雨水
 */

// @lc code=start
func trap(height []int) (ret int) {
	left, right := 0, len(height) - 1
	leftMax, rightMax := 0, 0
	for left < right {
		leftMax = max(leftMax, height[left])
		rightMax = max(rightMax, height[right])
		if height[left] < height[right] {
			ret += leftMax - height[left]
			left++
		} else {
			ret += rightMax - height[right]
			right--
		}
	}
	return
}
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
// @lc code=end

