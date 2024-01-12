/*
 * @lc app=leetcode.cn id=20 lang=golang
 *
 * [20] 有效的括号
 */

// @lc code=start
func isValid(s string) bool {
	length := len(s)
	if length%2 != 0 {
		return false
	}

	m := map[byte]byte{
		')': '(',
		'}': '{',
		']': '[',
	}
	tmp := []byte{}
	for i := 0; i < length; i++ {
		if len(tmp) > 0 && tmp[len(tmp)-1] == m[s[i]] {
			tmp = tmp[:len(tmp)-1]
		} else {
			tmp = append(tmp, s[i])
		}
	}
	return len(tmp) == 0
}

// @lc code=end

