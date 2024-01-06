/*
 * @lc app=leetcode.cn id=830 lang=golang
 *
 * [830] 较大分组的位置
 */

// @lc code=start
func largeGroupPositions(s string) (ret [][]int) {
	start := 0
	for start < len(s) {
		end := start + 1
		for ; end < len(s) && s[start] == s[end]; end++ {}
		if end - start >= 3 {
			ret = append(ret, []int{start, end-1})
		}
		start = end
	}
	return
}
// @lc code=end

