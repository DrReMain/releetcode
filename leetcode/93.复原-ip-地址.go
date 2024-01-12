/*
 * @lc app=leetcode.cn id=93 lang=golang
 *
 * [93] 复原 IP 地址
 */

// @lc code=start
import (
	"fmt"
	"strconv"
)
func restoreIpAddresses(s string) (ret []string) {
	count := 4
	segments := make([]int, count)
	var dfs func(string, int, int)
	dfs = func (s string, idx int, start int) {
		if idx == count {
			if start == len(s) {
				ret = append(ret, fmt.Sprintf("%d.%d.%d.%d", segments[0], segments[1], segments[2], segments[3]))
			}
			return
		}

		if start == len(s) {
			return
		}

		if s[start] == '0' {
			segments[idx] = 0
			dfs(s, idx+1, start+1)
			return
		}

		addr := 0
		for end := start; end < len(s); end++ {
			digit, err := strconv.Atoi(string(s[end]))
			if err != nil {
				break
			}
			addr = addr*10 + digit
			if addr > 0 && addr <= 0xFF {
				segments[idx] = addr
				dfs(s, idx+1, end+1)
			} else {
				break
			}
		}
	}
	dfs(s, 0, 0)
	return
}
// @lc code=end

