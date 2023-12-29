/*
 * @lc app=leetcode.cn id=11 lang=golang
 *
 * [11] 盛最多水的容器
 */

// @lc code=start
func maxArea(height []int) (cap int) {
	length := len(height)
	i, j := 0, length-1
	for i < j {
		area := min(height[i], height[j]) * (j - i)
		cap = max(cap, area)

		if height[i] < height[j] {
			i++
		} else {
			j--
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

