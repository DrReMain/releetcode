/*
 * @lc app=leetcode.cn id=2678 lang=golang
 *
 * [2678] 老人的数目
 */

// @lc code=start
func countSeniors(details []string) (ret int) {
	for _, s := range details {
		if s[11] > '6' {
			ret++
		}
		if s[11] == '6' && s[12] > '0' {
			ret++
		}
	}
	return
}

// @lc code=end

