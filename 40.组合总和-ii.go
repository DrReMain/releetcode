/*
 * @lc app=leetcode.cn id=40 lang=golang
 *
 * [40] 组合总和 II
 */

// @lc code=start
func combinationSum2(candidates []int, target int) (ret [][]int) {
	var backtrack func(int, int, []int)
	backtrack = func (idx, remain int, item []int)  {
		if remain < 0 {
			return
		}
		if remain == 0 {
			ret = append(ret, append([]int{}, item...))
			return
		}

		for i := idx; i < len(candidates); i++ {
			if i > idx && candidates[i] == candidates[i-1] {
				continue
			}
			newItem := append([]int{}, item...)
			backtrack(i+1, remain-candidates[i], append(newItem, candidates[i]))
		}
	}
	sort.Ints(candidates)
	backtrack(0, target, []int{})
	return
}
// @lc code=end

