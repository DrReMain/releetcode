/*
 * @lc app=leetcode.cn id=9 lang=golang
 *
 * [9] 回文数
 */

// @lc code=start
func isPalindrome(x int) bool {
	if x < 0 {
		return false
	}
	origin := x
	ret := 0
	for origin > 0 {
		ret = ret*10 + origin%10
		origin = origin / 10
	}

	return ret == x
}

// @lc code=end

