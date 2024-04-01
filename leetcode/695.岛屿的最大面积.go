/*
 * @lc app=leetcode.cn id=695 lang=golang
 *
 * [695] 岛屿的最大面积
 */

// @lc code=start
func maxAreaOfIsland(grid [][]int) (ret int) {
	m, n := len(grid), len(grid[0])
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if grid[i][j] == 1 {
				ret = max(ret, dfs(grid, i, j))
			}
		}
	}
	return
}
func dfs(grid [][]int, x, y int) int {
	m, n := len(grid), len(grid[0])	
	if x < 0 || y < 0 || x >= m || y >= n || grid[x][y] == 0 {
		return 0
	}

	grid[x][y] = 0
	return 1 + dfs(grid, x+1, y) + dfs(grid, x-1, y) + dfs(grid, x, y+1) + dfs(grid, x, y-1)
}
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
// @lc code=end

