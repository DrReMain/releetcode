/*
 * @lc app=leetcode.cn id=5 lang=golang
 *
 * [5] 最长回文子串
 */

// @lc code=start
func longestPalindrome(s string) string {
	start, end := 0, 0
	for i := 0; i < len(s); i++ {
		left1, right1 := ExpandCenterAround(s, i, i)
		left2, right2 := ExpandCenterAround(s, i, i + 1)
		if right1-left1>end-start {
			start, end = left1, right1
		}
		if right2-left2>end-start {
			start, end = left2, right2
		}
	}
	return s[start:end+1]
}
func ExpandCenterAround(s string, left, right int) (int, int) {
	for ; left >= 0 && right < len(s) && s[left] == s[right]; left, right = left-1, right+1 {
	}
	return left + 1, right - 1
}

// @lc code=end

