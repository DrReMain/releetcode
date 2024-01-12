/*
 * @lc app=leetcode.cn id=3 lang=golang
 *
 * [3] 无重复字符的最长子串
 */

// @lc code=start
func lengthOfLongestSubstring(s string) (ret int) {
	length := len(s)
	if length < 2 {
		return length
	}
	m := make(map[byte]bool, length)

	cur := 0
	for i, _ := range s {
		if i > 0 {
			m[s[i-1]] = false
		}
		for cur < length && !m[s[cur]] {
			m[s[cur]] = true
			cur++
		}
		ret = max(ret, cur-i)
	}
	return
}
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

// @lc code=end

