/*
 * @lc app=leetcode.cn id=46 lang=golang
 *
 * [46] 全排列
 */

// @lc code=start
func permute(nums []int) (ret [][]int) {
	var backtrack func([]int)
	backtrack = func(path []int) {
		if len(path) == len(nums) {
			ret = append(ret, path)
			return
		}

		for i := 0; i < len(nums); i++ {
			if contains(path, nums[i]) {
				continue
			}
			backtrack(append(path, nums[i]))
		}
	}
	backtrack([]int{})
	return
}
func contains(arr []int, target int) bool {
	for _, v := range arr {
		if v == target {
			return true
		}
	}
	return false
}
// @lc code=end

