/*
 * @lc app=leetcode.cn id=59 lang=golang
 *
 * [59] 螺旋矩阵 II
 */

// @lc code=start
func generateMatrix(n int) [][]int {
	type direction struct { x, y int }
	directions := []direction{{0, 1}, {1, 0}, {0, -1}, {-1, 0}}
	ret := make([][]int, n)
	for i, _ := range ret  {
		ret[i] = make([]int, n)
	}
	x, y, dir := 0, 0, 0
	for i := 1; i <= n*n; i++ {
		ret[x][y] = i
		nextX, nextY := x+directions[dir].x, y+directions[dir].y
		if nextX >= n || nextX < 0 ||
			nextY >= n || nextY < 0 ||
			ret[nextX][nextY] != 0 {
				dir = (dir+1)%len(directions)
			}
		x += directions[dir].x
		y += directions[dir].y
	}
	return ret
}
// @lc code=end

