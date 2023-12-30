/*
 * @lc app=leetcode.cn id=14 lang=golang
 *
 * [14] 最长公共前缀
 */

// @lc code=start
func longestCommonPrefix(strs []string) string {
	if len(strs) == 0 {
		return ""
	}
	sort.Strings(strs)
	start, end := strs[0], strs[len(strs)-1]
	i := 0
	for ; i < len(start) && i < len(end) && start[i] == end[i]; i++ {}
	return start[:i]
}
// @lc code=end

