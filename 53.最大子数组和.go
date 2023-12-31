/*
 * @lc app=leetcode.cn id=53 lang=golang
 *
 * [53] 最大子数组和
 */

// @lc code=start
func maxSubArray(nums []int) int {
	pre, ret := 0, nums[0]
	for i := 0; i < len(nums); i++ {
		pre = max(pre+nums[i], nums[i])
		ret = max(ret, pre)
	}
	return ret
}
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
// @lc code=end

