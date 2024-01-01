/*
 * @lc app=leetcode.cn id=22 lang=golang
 *
 * [22] 括号生成
 */

// @lc code=start
func generateParenthesis(n int) (ret []string) {
	var backtrack func([]string, int, int)
	backtrack = func(s []string, left, right int) {
		if len(s) == n * 2 {
			ret = append(ret, strings.Join(s, ""))
			return
		}
		if left < n {
			backtrack(append(s, "("), left+1, right)
		}
		if right < left {
			backtrack(append(s, ")"), left, right+1)
		}
	}
	backtrack([]string{}, 0, 0)
	return
}
// @lc code=end

