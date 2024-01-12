/*
 * @lc app=leetcode.cn id=200 lang=golang
 *
 * [200] 岛屿数量
 */

// @lc code=start
func numIslands(grid [][]byte) (ret int) {
	if len(grid) == 0 {
		return 0
	}
	row, col := len(grid), len(grid[0])
	for i := 0; i < row; i++ {
		for j := 0; j < col; j++ {
			if grid[i][j] == '1' {
				ret++
				dfs(grid, i, j)
			}
		}
	}
	return
}
func dfs(grid [][]byte, i, j int) {
	row, col := len(grid), len(grid[0])
	if i < 0 || j < 0 || i >= row || j >= col || grid[i][j] == '0' {
		return
	}
	grid[i][j] = '0'
	dfs(grid, i+1, j)
	dfs(grid, i-1, j)
	dfs(grid, i, j+1)
	dfs(grid, i, j-1)
}
// @lc code=end

