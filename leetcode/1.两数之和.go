/*
 * @lc app=leetcode.cn id=1 lang=golang
 *
 * [1] 两数之和
 */

// @lc code=start
func twoSum(nums []int, target int) []int {
	m := map[int]int{}
	for idx, n := range nums {
		if n, ok := m[target-n]; ok {
			return []int{n, idx}
		}
		m[n] = idx
	}
	return nil
}

// @lc code=end

