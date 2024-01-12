/*
 * @lc app=leetcode.cn id=26 lang=golang
 *
 * [26] 删除有序数组中的重复项
 */

// @lc code=start
func removeDuplicates(nums []int) int {
	length := len(nums)
	m := make(map[int]bool, length)
	cur := -1
	for i := 0; i < length; i++ {
		if v, _ := m[nums[i]]; !v {
			cur++
			nums[cur] = nums[i]
			m[nums[i]] = true
		}
	}
	return cur + 1
}

// @lc code=end

