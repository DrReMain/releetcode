/*
 * @lc app=leetcode.cn id=2696 lang=golang
 *
 * [2696] 删除子串后的字符串最小长度
 */

// @lc code=start
func minLength(s string) int {
	stack := []byte{}
	for i := range s {
		stack = append(stack, s[i])
		l := len(stack)
		if l >= 2 && (string(stack[l-2:]) == "AB" || string(stack[l-2:]) == "CD") {
			stack = stack[:l-2]
		}
	}
	return len(stack)
}
// @lc code=end

