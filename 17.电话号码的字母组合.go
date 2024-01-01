/*
 * @lc app=leetcode.cn id=17 lang=golang
 *
 * [17] 电话号码的字母组合
 */

// @lc code=start
func letterCombinations(digits string) (ret []string) {
	if len(digits) == 0 {
		return
	}
	m := map[byte][]string {
		'2': {"a", "b", "c"},
		'3': {"d", "e", "f"},
		'4': {"g", "h", "i"},
		'5': {"j", "k", "l"},
		'6': {"m", "n", "o"},
		'7': {"p", "q", "r", "s"},
		'8': {"t", "u", "v"},
		'9': {"w", "x", "y", "z"},
	}
	var recur func([]string, int)
	recur = func(s []string, idx int) {
		if len(s) == len(digits) {
			ret = append(ret, strings.Join(s, ""))
			return
		}
		if idx < len(digits) {
			charList, _ := m[digits[idx]]
			for i := 0; len(charList) > 0 && i < len(charList); i++ {
				recur(append(s, charList[i]), idx+1)
			}
		}
	}
	recur([]string{}, 0)
	return
}
// @lc code=end

