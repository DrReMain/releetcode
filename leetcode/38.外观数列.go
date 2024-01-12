/*
 * @lc app=leetcode.cn id=38 lang=golang
 *
 * [38] 外观数列
 */

// @lc code=start
func countAndSay(n int) string {
	ret := "1"
	for i := 2; i <= n; i++ {
		var p, start int
		tmp := []string{}
		for p < len(ret) {
			for ;p < len(ret) && ret[p] == ret[start]; p++ {}
			tmp = append(tmp, fmt.Sprintf("%d%c", p-start, ret[start]))
			start = p
		}
		ret = strings.Join(tmp, "")
	}
	return ret
}
// @lc code=end

