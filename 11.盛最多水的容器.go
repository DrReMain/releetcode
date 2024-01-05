/*
 * @lc app=leetcode.cn id=11 lang=golang
 *
 * [11] 盛最多水的容器
 */

// @lc code=start
func maxArea(height []int) (cap int) {
	left, right := 0, len(height) - 1
	for left < right {
		area := min(height[left], height[right]) * (right-left)
		cap = max(cap, area)
		if height[left] < height[right] {
			left++
		} else {
			right--
		}
	}
	return
}
func min(a, b int) int {
	if a > b {
		return b
	}
	return a
}
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

// @lc code=end

