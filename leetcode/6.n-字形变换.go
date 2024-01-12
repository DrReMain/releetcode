/*
 * @lc app=leetcode.cn id=6 lang=golang
 *
 * [6] N 字形变换
 */

// @lc code=start
func convert(s string, numRows int) string {
	if numRows <= 1 {
		return s
	}

	ret := make([]string, numRows)
	dir, flag := 0, 1
	for i := 0; i < len(s); i++ {
		ret[dir] += string(s[i])

		nextDir := dir + flag
		if nextDir >= numRows || nextDir < 0 {
			flag *= -1
		}
		dir += flag
	}

	return strings.Join(ret, "")
}

// @lc code=end

