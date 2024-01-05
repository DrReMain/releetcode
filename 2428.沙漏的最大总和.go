/*
 * @lc app=leetcode.cn id=2428 lang=golang
 *
 * [2428] 沙漏的最大总和
 */

// @lc code=start
func maxSum(grid [][]int) (ret int) {
	m, n := len(grid), len(grid[0])
	for i := 1; i < m - 1; i++ {
		for j := 1; j < n - 1; j++ {
			tmp := grid[i][j] +
					grid[i-1][j] +
					grid[i+1][j] +
					grid[i-1][j-1] + 
					grid[i-1][j+1] +
					grid[i+1][j-1] +
					grid[i+1][j+1]
			ret = max(ret, tmp)
		}
	}
	return
}
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
// @lc code=end

