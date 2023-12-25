/*
 * @lc app=leetcode.cn id=27 lang=golang
 *
 * [27] 移除元素
 */

// @lc code=start
func removeElement(nums []int, val int) int {
	right, i := -1, 0
	for i < len(nums) {
		if nums[i] != val {
			right++
			nums[right] = nums[i]
		}
		i++
	}
	return right + 1
}

// @lc code=end

