/*
 * @lc app=leetcode.cn id=2661 lang=golang
 *
 * [2661] 找出叠涂元素
 */

// @lc code=start
func firstCompleteIndex(arr []int, mat [][]int) (k int) {
	m, n := len(mat), len(mat[0])

	type Tuple struct { x, y int}
	matrix := make(map[int]Tuple, m * n)
	rows := make([]int, m)
	cols := make([]int, n)
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			matrix[mat[i][j]] = Tuple{i, j}
		}
	}

	for k < m * n {
		t := matrix[arr[k]]
		if rows[t.x] + 1 == n || cols[t.y] + 1 == m {
			break
		} else {
			rows[t.x]++
			cols[t.y]++
		}
		k++
	}
	return
}
// @lc code=end

