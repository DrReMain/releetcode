/*
 * @lc app=leetcode.cn id=39 lang=golang
 *
 * [39] 组合总和
 */

// @lc code=start
func combinationSum(candidates []int, target int) (ret [][]int) {
	var backtrack func([]int, int, int)
	backtrack = func (item []int, remain int, idx int) {
		if idx >= len(candidates) {
			return
		}
		if remain == 0 {
			newItem := append([]int{}, item...)
			ret = append(ret, newItem)
			return
		}

		backtrack(item, remain, idx+1)
		
		if remain - candidates[idx] >= 0 {
			newItem := append([]int{}, item...)
			backtrack(append(newItem, candidates[idx]), remain-candidates[idx], idx)
		}
	}
	backtrack([]int{}, target, 0)
	return
}
// @lc code=end

