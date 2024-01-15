package n200

func numIslands(grid [][]byte) (ret int) {
	if len(grid) == 0 {
		return 0
	}
	row, col := len(grid), len(grid[0])
	for i := 0; i < row; i++ {
		for j := 0; j < col; j++ {
			if grid[i][j] == '1' {
				ret++
				numIslandsDfs(grid, i, j)
			}
		}
	}
	return
}
func numIslandsDfs(grid [][]byte, i, j int) {
	row, col := len(grid), len(grid[0])
	if i < 0 || j < 0 || i >= row || j >= col || grid[i][j] == '0' {
		return
	}
	grid[i][j] = '0'
	numIslandsDfs(grid, i+1, j)
	numIslandsDfs(grid, i-1, j)
	numIslandsDfs(grid, i, j+1)
	numIslandsDfs(grid, i, j-1)
}
