/*
 * @lc app=leetcode.cn id=338 lang=golang
 *
 * [338] 比特位计数
 */

// @lc code=start
func countBits(n int) []int {
	ans := make([]int, n+1)
	for i := 0; i <= n; i++ {
		remain, count := i, 0;
		for remain > 0 {
			if remain % 2 != 0 {
				count++
			}
			remain >>= 1
		}

		ans[i] = count
	}
	return ans
}
// @lc code=end

