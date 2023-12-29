/*
 * @lc app=leetcode.cn id=13 lang=golang
 *
 * [13] 罗马数字转整数
 */

// @lc code=start
func romanToInt(s string) (ret int) {
	m := map[byte]int{
		'I': 1,
		'V': 5,
		'X': 10,
		'L': 50,
		'C': 100,
		'D': 500,
		'M': 1000,
	}
	for i := 0; i < len(s); i++ {
		n, _ := m[s[i]]
		if i + 1 < len(s) && n < m[s[i+1]] {
			n = m[s[i+1]] - n
			i++
		}
		ret += n
	}
	return
}

// @lc code=end

