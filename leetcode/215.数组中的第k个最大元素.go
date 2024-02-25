/*
 * @lc app=leetcode.cn id=215 lang=golang
 *
 * [215] 数组中的第K个最大元素
 */

// @lc code=start
func findKthLargest(nums []int, k int) int {
	n := len(nums)
	return quickSelect(nums, 0, n-1, n-k)
}
func quickSelect(nums []int, l, r, k int) int {
	if l == r {
		return nums[k]
	}
	p, i, j := nums[l], l-1, r+1
	for i < j {
		for i++; nums[i] < p; i++ {}
		for j--; nums[j] > p; j-- {}
		if i < j {
			nums[i], nums[j] = nums[j], nums[i]
		}
	}

	if k <= j {
		return quickSelect(nums, l, j, k)
	} else {
		return quickSelect(nums, j+1, r, k)
	}
}
// @lc code=end

